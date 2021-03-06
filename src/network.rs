use std::io;
use std::net::ToSocketAddrs;

use may::coroutine::JoinHandle;
use may::net::{TcpListener, TcpStream};
use tungstenite::client::client;
use tungstenite::handshake::client::Request;
use tungstenite::server::accept;
use tungstenite::{Message, WebSocket};
use url::Url;

pub fn run_websocket_server<T: ToSocketAddrs>(address: T) -> JoinHandle<()> {
    let address = address
        .to_socket_addrs()
        .expect("invalid address")
        .next()
        .expect("can't resolve address");

    go!(move || {
        let listener = TcpListener::bind(address).unwrap();
        // for stream in listener.incoming() {
        while let Ok((stream, _)) = listener.accept() {
            go!(move || -> () {
                let mut websocket = accept(stream).expect("ws failed to accept");

                loop {
                    let msg = match websocket.read_message() {
                        Ok(msg) => msg,
                        Err(e) => {
                            error!("{}", e.to_string());
                            break;
                        }
                    };

                    // Just echo back everything that the client sent to us
                    if msg.is_binary() || msg.is_text() {
                        websocket.write_message(msg).expect("ws failed to write");
                    }
                }
            });
        }
    })
}

pub struct WsClient {
    client: WebSocket<TcpStream>,
}

impl WsClient {
    pub fn new<T: ToSocketAddrs>(address: T) -> io::Result<Self> {
        let stream = TcpStream::connect(address)?;
        let url = Url::parse("ws://localhost:8080/").unwrap();
        let req = Request::from(url);
        match client(req, stream) {
            Ok((client, _)) => Ok(WsClient { client }),
            Err(_) => Err(io::Error::new(
                io::ErrorKind::NotConnected,
                "failed ws handshake",
            )),
        }
    }

    pub fn send_message(&mut self, msg: String) -> io::Result<()> {
        self.client
            .write_message(Message::Text(msg))
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
    }

    pub fn recv_message(&mut self) -> io::Result<String> {
        let msg = self.client
            .read_message()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        match msg {
            Message::Text(s) => Ok(s),
            _ => Err(io::Error::new(io::ErrorKind::Other, "not a text message")),
        }
    }
}
