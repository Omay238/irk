use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net;

pub struct IRCClient {
    reader: io::ReadHalf<net::TcpStream>,
    writer: io::WriteHalf<net::TcpStream>,
}

impl IRCClient {
    pub async fn new(url: String) -> Self {
        let (reader, writer) = io::split(
            net::TcpStream::connect(url)
                .await
                .expect("failed to connect!"),
        );

        Self { reader, writer }
    }

    pub async fn connect(&mut self, nick: String, name: String) {
        self.writer
            .write(format!("NICK {nick}\r\n").as_bytes())
            .await
            .expect("failed to send nick!");
        self.writer
            .write(format!("USER guest 0 * :{name}\r\n").as_bytes())
            .await
            .expect("failed to send real name!");
    }

    pub async fn listen(&mut self) {
        loop {
            let mut buf = [0u8; 512];
            match self.reader.read(&mut buf).await {
                Ok(0) => {
                    println!("server disconnected");
                    break;
                }
                Ok(n) => {
                    print!("{}", String::from_utf8_lossy(&buf[..n]));
                }
                Err(e) => {
                    println!("read error: {e}");
                    break;
                }
            }
        }
    }
}
