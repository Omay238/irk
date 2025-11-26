use std::ops::Deref;
use tokio::io::{self, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let stream = TcpStream::connect(format!(
            "{}:6667",
            std::env::args()
                .nth(1)
                .unwrap_or(String::from("irc.hackclub.com"))
        ))
        .await
        .expect("failed to connect!");

    let (reader, writer) = io::split(stream);

    let mut reader = Arc::new(Mutex::new(reader));
    let mut writer = Arc::new(Mutex::new(writer));

    let nick = "owomay";
    let name = "owomay";

    writer
        .lock()
        .await
        .write(format!("NICK {nick}\r\n").as_bytes())
        .await
        .expect("failed to send message!");
    writer
        .lock()
        .await
        .write(format!("USER guest 0 * :{name}\r\n").as_bytes())
        .await
        .expect("failed to send message!");

    let auto_writer = writer.clone();
    let mut stop = false;

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
                        auto_writer
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
                        print!("{parsed}");
                    }
                }
                Err(e) => {
                    println!("read error: {e}");
                    break;
                }
            }
        }
    });

    let mut channel = String::new();

    loop {
        let mut input = String::new();

        std::io::stdin().read_line(&mut input).unwrap();

        if input.starts_with("/") {
            if input.starts_with("/join") {
                if channel != "" {
                    writer
                        .lock()
                        .await
                        .write(format!("PART {}\r\n", channel).as_bytes())
                        .await
                        .expect("failed to send message!");
                }
                channel = String::from(input.split(" ").nth(1).unwrap().trim());
                writer
                    .lock()
                    .await
                    .write(format!("JOIN {}\r\n", channel).as_bytes())
                    .await
                    .expect("failed to send message!");
            } else if input.starts_with("/part") {
                if channel != "" {
                    writer
                        .lock()
                        .await
                        .write(format!("PART {} :{}\r\n", channel, input.chars().skip(6).collect::<String>()).as_bytes())
                        .await
                        .expect("failed to send message!");
                }
            } else if input.starts_with("/quit") {
                writer
                    .lock()
                    .await
                    .write(format!("QUIT :{}\r\n", input.chars().skip(6).collect::<String>()).as_bytes())
                    .await
                    .expect("failed to send message!");
            } else if input.starts_with("/nick") {
                writer
                    .lock()
                    .await
                    .write(format!("NICK {}\r\n", input.chars().skip(6).collect::<String>()).as_bytes())
                    .await
                    .expect("failed to send message!");
            }
        } else if channel != "" {
            println!("PRIVMSG {} :{}\r\n", channel, input.trim());
            writer
                .lock()
                .await
                .write(format!("PRIVMSG {} :{}\r\n", channel, input.trim()).as_bytes())
                .await
                .expect("failed to send message!");
        }
    }
}
