use irk::*;

#[tokio::main]
async fn main() {
    let mut irc = IRCClient::new(String::from(
        std::env::args()
            .nth(1)
            .unwrap_or(String::from("irc.hackclub.com:6667")),
    ))
    .await;
    irc.connect(String::from("owomay"), String::from("owomay"))
        .await;
    irc.listen().await;
}
