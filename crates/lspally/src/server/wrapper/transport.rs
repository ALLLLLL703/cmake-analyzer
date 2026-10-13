// AI-generated implementation.
use crate::server::model::{Cancellation, Limits, WritePacket};
use std::io;
use tokio::{
    io::{AsyncBufRead, AsyncBufReadExt, AsyncReadExt, AsyncWrite, AsyncWriteExt},
    sync::mpsc,
};

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
fn invalid(error: impl ToString) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, error.to_string())
}

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
