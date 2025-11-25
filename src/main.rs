use irk::*;

#[tokio::main]
async fn main() {
    let mut irc = IRCClient::new(format!(
        "{}:6667",
        std::env::args()
            .nth(1)
            .unwrap_or(String::from("irc.hackclub.com")),
    ))
    .await;
    irc.connect(String::from("owomay"), String::from("owomay"))
        .await;
    irc.listen().await;
}
