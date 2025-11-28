use libirk::*;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let username = std::env::args().nth(1).unwrap();
    let realname = std::env::args().nth(2).unwrap();

    let irc = Arc::new(
        Mutex::new(
            IRCClient::new(
                std::env::args()
                    .nth(3)
                    .unwrap_or(String::from("irc.hackclub.com:6667")),
            )
            .await
            .expect("connection failed"),
        )
    );

    let stop = Arc::new(Mutex::new(false));
    let channel: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));

    let _handle = {
        let client = irc.clone();
        let user = username.clone();
        let real = realname.clone();
        let chan = channel.clone();
        client
            .lock()
            .await
            .listen(Box::new(move |msg: String| {
                if msg.starts_with(format!(":{}!{}", user, real).as_str()) {
                    let msg_chan = msg.split(' ').nth(2).unwrap().split('\r').next().unwrap().trim().trim_matches(':').to_string();
                    let chan_clone = chan.clone();
                    tokio::spawn(async move {
                        let mut channel_lock = chan_clone.lock().await;
                        *channel_lock = Some(msg_chan);
                    });
                }
                print!("{}", msg);
            }))
            .await
    };

    irc
        .lock()
        .await
        .connect(username, realname)
        .await;

    loop {
        if *stop.lock().await == true {
            break;
        }

        let mut input = String::new();

        std::io::stdin().read_line(&mut input).unwrap();

        irc
            .lock()
            .await
            .send_user_message(input, channel.lock().await.clone())
            .await;
    }
}
