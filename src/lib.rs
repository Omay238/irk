use std::sync::Arc;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net;
use tokio::sync::Mutex;

pub enum Message {}
impl ToString for Message {
    fn to_string(&self) -> String {
        todo!()
    }
}

/// A simple IRC Client
pub struct IRCClient {
    reader: Arc<Mutex<io::ReadHalf<net::TcpStream>>>,
    writer: Arc<Mutex<io::WriteHalf<net::TcpStream>>>,
}

impl IRCClient {
    /// Create a new IRC Client to a specific server address
    pub async fn new(url: String) -> io::Result<Self> {
        let (reader, writer) = io::split(net::TcpStream::connect(url).await?);

        let reader = Arc::new(Mutex::new(reader));
        let writer = Arc::new(Mutex::new(writer));

        Ok(Self { reader, writer })
    }

    /// Register a connection to the server
    pub async fn connect(&mut self, nick: String, name: String) {
        self.send_message(format!("NICK {nick}\r\n")).await;
        self.send_message(format!("USER {nick} 0 * :{name}\r\n"))
            .await;
    }

    /// Begin the listening loop
    pub async fn listen(
        &mut self,
        callback: Box<dyn Fn(String) + Send + 'static>,
    ) -> tokio::task::JoinHandle<()> {
        let reader = self.reader.clone();
        let writer = self.writer.clone();

        tokio::spawn(async move {
            loop {
                let mut buf = [0u8; 512];
                match reader.lock().await.read(&mut buf).await {
                    Ok(0) => {
                        println!("server disconnected");
                        break;
                    }
                    Ok(n) => {
                        let parsed = String::from_utf8_lossy(&buf[..n]);
                        if parsed.starts_with("PING") {
                            writer
                                .lock()
                                .await
                                .write(
                                    format!(
                                        "PONG {}\r\n",
                                        parsed.split(" ").last().expect("invalid ping")
                                    )
                                    .as_bytes(),
                                )
                                .await
                                .expect("failed to send message!");
                        } else {
                            callback(String::from(parsed));
                        }
                    }
                    Err(e) => {
                        println!("read error: {e}");
                        break;
                    }
                }
            }
        })
    }

    /// Take user command and send to server
    pub async fn send_user_message(&mut self, content: String) {
        self.send_message(format!("{}\r\n", content.trim())).await;
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
