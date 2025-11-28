use irk::*;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let irc = Arc::new(Mutex::new(
        IRCClient::new(
            std::env::args()
                .nth(1)
                .unwrap_or(String::from("irc.hackclub.com:6667")),
        )
        .await
        .expect("connection failed"),
    ));

    let stop = Arc::new(Mutex::new(false));

    let _handle = {
        let client = irc.clone();
        client
            .lock()
            .await
            .listen(Box::new(|msg: String| {
                print!("{}", msg);
            }))
            .await
    };

    irc.lock()
        .await
        .connect(String::from("owomay"), String::from("owomay"))
        .await;

    loop {
        if *stop.lock().await == true {
            break;
        }

        let mut input = String::new();

        std::io::stdin().read_line(&mut input).unwrap();

        irc.lock().await.send_user_message(input).await;
    }
}
