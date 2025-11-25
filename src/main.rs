use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::{io, net};

#[tokio::main]
async fn main() {
    let (mut reader, mut writer) = io::split(
        net::TcpStream::connect(format!(
            "{}:6667",
            std::env::args()
                .nth(1)
                .unwrap_or(String::from("irc.hackclub.com"))
        ))
        .await
        .expect("failed to connect!"),
    );

    let nick = "owomay";
    let name = "owomay";

    writer
        .write(format!("NICK {nick}\r\n").as_bytes())
        .await
        .expect("failed to send message!");
    writer
        .write(format!("USER guest 0 * :{name}\r\n").as_bytes())
        .await
        .expect("failed to send message!");

    tokio::spawn(async move {
        loop {
            let mut buf = [0u8; 512];
            match reader.read(&mut buf).await {
                Ok(0) => {
                    println!("server disconnected");
                    break;
                }
                Ok(n) => {
                    let parsed = String::from_utf8_lossy(&buf[..n]);
                    if parsed.starts_with("PING") {
                        writer
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
    loop {}
}
