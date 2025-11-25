use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net;

enum Message {
    JOIN(String),
    PART(String),
}

/// A simple IRC Client
pub struct IRCClient {
    reader: io::ReadHalf<net::TcpStream>,
    writer: io::WriteHalf<net::TcpStream>,
}

impl IRCClient {
    /// Create a new IRC Client to a specific server address
    pub async fn new(url: String) -> Self {
        let (reader, writer) = io::split(
            net::TcpStream::connect(url)
                .await
                .expect("failed to connect!"),
        );

        Self { reader, writer }
    }

    /// Register a connection to the server
    pub async fn connect(&mut self, nick: String, name: String) {
        self.send_message(format!("NICK {nick}\r\n")).await;
        self.send_message(format!("USER guest 0 * :{name}\r\n"))
            .await;
    }

    /// Begin the listening loop
    pub async fn listen(&mut self) {
        loop {
            let mut buf = [0u8; 512];
            match self.reader.read(&mut buf).await {
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
                    print!("{}", String::from_utf8_lossy(&buf[..n]));
                }
                Err(e) => {
                    println!("read error: {e}");
                    break;
                }
            }
        }
    }

    /// Take a user space command and convert it to an IRC command and send it.
    pub async fn send_command(&mut self, content: String) {}

    /// Take an IRC command and convert it to an understandable Message enum
    async fn parse_command(&mut self, content: String) -> Message {
        Message::JOIN(String::new())
    }

    /// Helper function to easily send content to the server
    pub async fn send_message(&mut self, content: String) {
        self.writer
            .write(content.as_bytes())
            .await
            .expect("failed to send message!");
    }
}
