use irk::*;

fn main() {
    let mut irc = IRCClient::new(String::from(
        std::env::args()
            .nth(1)
            .unwrap_or(String::from("irc.hackclub.com:6667")),
    ));
    irc.connect(String::from("owomay"), String::from("owomay"));
    irc.listen();
}
