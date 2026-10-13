// AI-generated test helper.
use super::fixture::Server;
use lspally::server::{ExitStatus, Wrapper};
use serde_json::{Value, json};
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader, DuplexStream, ReadHalf, WriteHalf},
    task::JoinHandle,
};

pub(super) struct Session {
    input: BufReader<ReadHalf<DuplexStream>>,
    output: WriteHalf<DuplexStream>,
    pub events: Arc<Mutex<Vec<String>>>,
    task: JoinHandle<std::io::Result<ExitStatus>>,
}
impl Session {
    pub fn new() -> Self {
        let (client, server) = tokio::io::duplex(65536);
        let (input, output) = tokio::io::split(client);
        let (reader, writer) = tokio::io::split(server);
        let events = Arc::new(Mutex::new(Vec::new()));
        let wrapper = Wrapper::new(Server {
            events: events.clone(),
        });
        Self {
            input: BufReader::new(input),
            output,
            events,
            task: tokio::spawn(wrapper.run(reader, writer)),
        }
    }
    pub async fn send(&mut self, value: Value) {
        self.raw(&value.to_string()).await;
    }
    pub async fn raw(&mut self, body: &str) {
        self.output
            .write_all(format!("Content-Length: {}\r\n\r\n{body}", body.len()).as_bytes())
            .await
            .unwrap();
    }
    pub async fn receive(&mut self) -> Value {
        tokio::time::timeout(Duration::from_secs(3), async {
            let mut header = Vec::new();
            while !header.ends_with(b"\r\n\r\n") {
                header.push(self.input.read_u8().await.unwrap());
            }
            let header = String::from_utf8(header).unwrap();
            let length: usize = header
                .strip_prefix("Content-Length: ")
                .unwrap()
                .trim()
                .parse()
                .unwrap();
            let mut body = vec![0; length];
            self.input.read_exact(&mut body).await.unwrap();
            serde_json::from_slice(&body).unwrap()
        })
        .await
        .expect("expected an LSP frame")
    }
    pub async fn initialize(&mut self) {
        self.send(json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"processId":null,"capabilities":{"workspace":{"configuration":true},"general":{"positionEncodings":["utf-8","utf-16"]}}}})).await;
        let result = self.receive().await;
        assert_eq!(
            result["result"]["capabilities"]["positionEncoding"],
            "utf-8"
        );
        self.send(json!({"jsonrpc":"2.0","method":"initialized","params":{}}))
            .await;
        let request = self.receive().await;
        assert_eq!(request["method"], "workspace/configuration");
        self.send(json!({"jsonrpc":"2.0","id":request["id"],"result":["configured"]}))
            .await;
    }
    pub async fn query(&mut self, id: i32, method: &str, params: Value) -> Value {
        self.send(json!({"jsonrpc":"2.0","id":id,"method":method,"params":params}))
            .await;
        let response = self.receive().await;
        assert_eq!(response["id"], id);
        response
    }
    pub async fn shutdown(&mut self) {
        let response = self.query(999, "shutdown", Value::Null).await;
        assert!(response.get("result").unwrap().is_null());
        self.send(json!({"jsonrpc":"2.0","method":"exit"})).await;
    }
    pub async fn await_stop(mut self) -> std::io::Result<ExitStatus> {
        tokio::time::timeout(Duration::from_secs(3), &mut self.task)
            .await
            .expect("server task must stop without client EOF")
            .expect("server task joined")
    }
    pub async fn finish(self) -> std::io::Result<ExitStatus> {
        let Self {
            input,
            output,
            task,
            ..
        } = self;
        drop(input);
        drop(output);
        tokio::time::timeout(Duration::from_secs(3), task)
            .await
            .expect("server task must stop")
            .expect("server task joined")
    }
}
