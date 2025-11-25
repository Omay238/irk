use std::io::prelude::*;
use std::net::TcpStream;

pub struct IRCClient {
    stream: TcpStream,
}

impl IRCClient {
    pub fn new(url: String) -> Self {
        let stream = TcpStream::connect(url).expect("failed to connect!");

        Self { stream }
    }

    pub fn connect(&mut self, nick: String, name: String) {
        self.stream
            .write(format!("NICK {nick}\r\n").as_bytes())
            .expect("failed to send nickname!");
        self.stream
            .write(format!("USER guest 0 * :{name}\r\n").as_bytes())
            .expect("failed to send real name!");
    }

    pub fn listen(&mut self) {
        let mut buf = [0u8; 512];
        loop {
            match self.stream.read(&mut buf) {
                Ok(0) => {
                    println!("server disconnected");
                    break;
                }
                Ok(n) => {
                    println!("{}", String::from_utf8_lossy(&buf[..n]));
                }
                Err(e) => {
                    println!("read error: {e}");
                    break;
                }
            }
        }
    }
}
