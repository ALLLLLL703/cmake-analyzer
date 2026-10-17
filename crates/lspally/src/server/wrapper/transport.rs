// AI-generated implementation.
//! Bounded LSP frame input and a single flush-acknowledging output owner.
//!
//! Headers are ASCII with CRLF endings and exactly one Content-Length. Body bytes
//! are read exactly; JSON/UTF-8 validation is the runtime reader's responsibility.
//! Writer cancellation can interrupt a partially written frame; closure is not a drain.
use crate::server::model::{Cancellation, Limits, WritePacket};
use std::io;
use tokio::{
    io::{AsyncBufRead, AsyncBufReadExt, AsyncReadExt, AsyncWrite, AsyncWriteExt},
    sync::mpsc,
};

/// Consume one bounded LSP header/body from the reader; performs input I/O only.
/// Checks framing and declared charset, not body UTF-8 or JSON; the runtime decodes JSON later.
/// Unknown headers and Content-Type media types are otherwise ignored.
///
/// # Arguments
/// - `reader` `[in,out]`: buffered stream advanced through consumed headers and exactly the declared
///   body on success. Errors/cancellation can leave it partway through a frame; no rewind.
/// - `limits` `[in]`: maximum total header bytes (including CRLF and blank line) and body bytes.
///
/// # Returns
/// `Ok(Some(body))` with exactly Content-Length bytes, including an empty vector for length zero;
/// `Ok(None)` for EOF before any next-header byte; `Err` for malformed/truncated input or I/O failure.
///
/// # Errors
/// Returns InvalidData for header/body limits, non-ASCII/non-CRLF or colon-less header lines,
/// missing/duplicate/invalid Content-Length, or a charset other than UTF-8/utf8 when specified.
/// Returns UnexpectedEof for an incomplete header or body, and propagates reader errors.
///
/// # Algorithm
/// Accumulate bounded header lines across buffer chunks until the blank CRLF line. Track one
/// case-insensitive Content-Length, check declared sizes before body allocation, then read exactly
/// that many bytes. Charset matching is case-insensitive; omitted charset is accepted.
///
/// # Flow
/// ```text
///      +------> [read bounded CRLF line]
///      |                    |
///      |                 <EOF?>
///      |                 /    \
///      |      [None if fresh;  <blank line?>
///      |       error if not]    /        \
///      |                      no        yes
///      |                       |         |
///      +--- [validate header] -+   [require length]
///                                        |
///                               [allocate/read body]
///                                        |
///                               [Some bytes / error]
/// ```
/// # Examples
/// Private transport context; `Limits::default()` supplies the size bounds.
/// ```text
/// let bytes = b"Content-Length: 2\r\n\r\n{}";
/// let mut reader = tokio::io::BufReader::new(&bytes[..]);
/// assert_eq!(frame(&mut reader, &Limits::default()).await?, Some(b"{}".to_vec()));
/// ```
pub(super) async fn frame<R: AsyncBufRead + Unpin>(
    reader: &mut R,
    limits: &Limits,
) -> io::Result<Option<Vec<u8>>> {
    let mut total = 0;
    let mut length = None;
    loop {
        let mut line = Vec::new();
        loop {
            let available = reader.fill_buf().await?;
            if available.is_empty() {
                if total == 0 && line.is_empty() {
                    return Ok(None);
                }
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "Incomplete LSP header",
                ));
            }
            let used = available
                .iter()
                .position(|b| *b == b'\n')
                .map_or(available.len(), |i| i + 1);
            if total + used > limits.header_bytes {
                return Err(invalid("LSP header limit exceeded"));
            }
            line.extend_from_slice(&available[..used]);
            reader.consume(used);
            total += used;
            if line.last() == Some(&b'\n') {
                break;
            }
        }
        if !line.ends_with(b"\r\n") {
            return Err(invalid("LSP headers require CRLF"));
        }
        line.truncate(line.len() - 2);
        if line.is_empty() {
            break;
        }
        if !line.is_ascii() {
            return Err(invalid("LSP headers must be ASCII"));
        }
        let header = std::str::from_utf8(&line).map_err(invalid)?;
        let (name, value) = header
            .split_once(':')
            .ok_or_else(|| invalid("Malformed LSP header"))?;
        if name.eq_ignore_ascii_case("Content-Length") {
            if length.is_some() {
                return Err(invalid("Duplicate Content-Length"));
            }
            let bytes: usize = value.trim().parse().map_err(invalid)?;
            if bytes > limits.frame_bytes {
                return Err(invalid("LSP frame limit exceeded"));
            }
            length = Some(bytes);
        } else if name.eq_ignore_ascii_case("Content-Type") {
            for parameter in value.split(';').skip(1) {
                if let Some((name, encoding)) = parameter.trim().split_once('=') {
                    if name.eq_ignore_ascii_case("charset")
                        && !matches!(
                            encoding.trim().to_ascii_lowercase().as_str(),
                            "utf-8" | "utf8"
                        )
                    {
                        return Err(invalid("LSP requires UTF-8"));
                    }
                }
            }
        }
    }
    let length = length.ok_or_else(|| invalid("Missing Content-Length"))?;
    let mut body = vec![0; length];
    reader.read_exact(&mut body).await?;
    Ok(Some(body))
}
/// Convert a displayable framing/serialization cause into InvalidData; no I/O or mutation.
///
/// # Arguments
/// - `error` `[in]`: owned cause converted to its string description.
///
/// # Returns
/// An `io::Error` with kind `InvalidData` and the supplied cause text.
///
/// # Examples
/// ```text
/// assert_eq!(invalid("Missing Content-Length").kind(), std::io::ErrorKind::InvalidData);
/// ```
fn invalid(error: impl ToString) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, error.to_string())
}

/// Own serialized frame output and acknowledge each complete write/flush.
/// Writes protocol bytes only; cancellation stops promptly rather than draining queued packets.
///
/// # Arguments
/// - `output` `[in,out]`: owned stream receiving frames in queue order; normal queue exhaustion
///   calls `shutdown`. Cancellation/failure may leave a partial frame and does not call shutdown.
/// - `queue` `[in,out]`: owned receiver drained one packet at a time; packets/acknowledgement senders
///   still queued are dropped when this function exits.
/// - `closed` `[in,out]`: shared cancellation token observed at receive/write waits and set on write
///   or serialization failure; a shutdown failure after queue exhaustion does not set it here.
///
/// # Returns
/// `Ok(())` on cancellation or successful shutdown after queue exhaustion; `Err` on serialization,
/// write, flush or shutdown failure. Success on cancellation does not promise all packets were sent.
///
/// # Errors
/// Serialization failures become InvalidData; output failures propagate. For a selected packet,
/// sends `Ok(())` only after flush, or an error string on failure/cancellation. Dropped acknowledgement
/// receivers are ignored. Cancellation can win over a simultaneous write result.
///
/// # Algorithm
/// Race closure with queue receive, serialize the packet, then race closure with write-all plus
/// flush. Notify the packet's one-use sender before continuing or returning a write failure.
///
/// # Flow
/// ```text
///                [receive packet] <-----------------------+
///                  /     |     \                          |
///           closed    packet    queue ended              |
///              |         |          |                    |
///          [Ok stop] [serialize] [shutdown output]        |
///                        |                               |
///                <closed or write/flush?>                |
///                  /                \                    |
///       [ack error; Ok stop]     [ack result]             |
///                                   |                    |
///                              <failed?>                 |
///                             /         \                |
///                     [cancel; Err]     +----------------+
/// ```
/// # Examples
/// Private transport context; a Tokio executor is needed for the asynchronous calls.
/// ```text
/// let (sender, queue) = tokio::sync::mpsc::channel(1);
/// drop(sender); // empty queue terminates and shuts down the supplied output
/// writer(tokio::io::sink(), queue, Cancellation::default()).await?;
/// ```
pub(super) async fn writer<W: AsyncWrite + Unpin>(
    mut output: W,
    mut queue: mpsc::Receiver<WritePacket>,
    closed: Cancellation,
) -> io::Result<()> {
    loop {
        let packet = tokio::select! {
            _=closed.cancelled()=>return Ok(()),
            packet=queue.recv()=>match packet {Some(packet)=>packet,None=>break},
        };
        let write = async {
            match packet.message.display() {
                Ok(frame) => {
                    async {
                        output.write_all(frame.as_bytes()).await?;
                        output.flush().await
                    }
                    .await
                }
                Err(error) => Err(invalid(error)),
            }
        };
        let result = tokio::select! {
            _=closed.cancelled()=>{let _=packet.done.send(Err("LSP connection closed".into()));return Ok(());},
            result=write=>result,
        };
        let _ = packet
            .done
            .send(result.as_ref().map(|_| ()).map_err(ToString::to_string));
        if let Err(error) = result {
            closed.cancel();
            return Err(error);
        }
    }
    output.shutdown().await
}
