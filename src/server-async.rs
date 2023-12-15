use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{self, Receiver, Sender};

const PORT: i32 = 8000;

#[derive(Debug, Clone)]
struct Message {
    content: Vec<u8>,
    sender: SocketAddr,
}

struct Client {
    stream: TcpStream,
    addr: SocketAddr,
    sender: Sender<Message>,
    receiver: Receiver<Message>,
}

impl Message {
    fn as_string(&self) -> String {
        String::from_utf8(self.content.clone())
            .expect("error during string conversion")
            .trim()
            .to_owned()
    }
}

async fn handle_client(mut c: Client) {
    loop {
        let mut buf = [0; 256].to_vec();
        tokio::select! {
            m = c.stream.read(buf.as_mut_slice()) => {
                match m {
                    Ok(0) => {
                        println!("client {} disconnected", c.addr);
                        return;
                    }
                    Ok(len) => {
                        let sender = c.addr;
                        let message = Message {
                            content: (&buf[0..len]).to_vec(),
                            sender
                        };

                        println!("{} sent message {}\n", c.addr, message.as_string());

                        c.sender
                            .send(message)
                            .expect("error sending message to other clients");

                    }
                    Err(e) => {
                        println!("error reading message {:?}", e);
                    }
                }
            }
            m = c.receiver.recv() => {
                match m {
                    Ok(message) => {
                        c.stream.write(&message.content).await.expect("error sending messages to streams");
                    }
                    Err(_) => {
                        println!("recieve error");
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let listener = TcpListener::bind(format!("127.0.0.1:{PORT}"))
        .await
        .expect("error binding tcp listener to port");
    let (s, _) = broadcast::channel::<Message>(20);

    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
                let receiver = s.subscribe();
                let sender = s.clone();
                let c = Client {
                    stream,
                    addr,
                    sender,
                    receiver,
                };
                tokio::task::spawn(async move {
                    println!("client connected on {}\n", c.addr);
                    handle_client(c).await;
                });
            }
            Err(_) => {
                println!("error accepting connection");
            }
        }
    }
}
