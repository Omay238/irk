use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net;
use std::sync::Arc;
use tokio::sync::Mutex;

/// A simple IRC Client
pub struct IRCClient {
    reader: Arc<Mutex<io::ReadHalf<net::TcpStream>>>,
    writer: Arc<Mutex<io::WriteHalf<net::TcpStream>>>,
}

impl IRCClient {
    /// Create a new IRC Client to a specific server address
    pub async fn new(url: String) -> io::Result<Self> {
        let (reader, writer) = io::split(
            net::TcpStream::connect(url)
                .await?
        );

        let reader = Arc::new(Mutex::new(reader));
        let writer = Arc::new(Mutex::new(writer));

        Ok(Self { reader, writer })
    }

    /// Register a connection to the server
    pub async fn connect(&mut self, nick: String, name: String) {
        self.send_message(format!("NICK {nick}\r\n")).await;
        self.send_message(format!("USER guest 0 * :{name}\r\n")).await;
    }

    /// Begin the listening loop
    pub async fn listen(&mut self) {
        loop {
            let mut buf = [0u8; 512];
            let reader = self.reader.clone();
            match reader.lock().await.read(&mut buf).await {
                Ok(0) => {
                    println!("server disconnected");
                    break;
                }
                Ok(n) => {
                    let parsed = String::from_utf8_lossy(&buf[..n]);
                    if parsed.starts_with("PING") {
                        self.send_message(format!(
                            "PONG {}\r\n",
                                parsed.split(" ").last().expect("invalid ping")
                        ))
                        .await;
                    }
                }
                Err(e) => {
                    println!("read error: {e}");
                    break;
                }
            }
        }
    }

    /// Helper function to easily send content to the server
    pub async fn send_message(&mut self, content: String) {
        self.writer
            .lock()
            .await
            .write(content.as_bytes())
            .await
            .expect("failed to send message!");
    }
}
