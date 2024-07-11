/// TODO: the code below will deadlock because it's using std's channels,
///  which are not async-aware.
///  Rewrite it to use `tokio`'s channels primitive (you'll have to touch
///  the testing code too, yes).
///
/// Can you understand the sequence of events that can lead to a deadlock?
// use std::sync::mpsc;
use tokio::sync::mpsc;

// Bookの方の序盤のプログラム、コンパイル通ると言っているけど、async fn mainで実際に使おうとするとSend不足でエラーになる
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=6b9ff64f9c4c94d0c0047de027ca1858

// 「リスク回避の観点では非同期プリミティブを優先しましょう」と書いているが、
// パフォーマンスを考慮し std::sync::Mutex を良しとする考えのチュートリアルもある
// https://zenn.dev/magurotuna/books/tokio-tutorial-ja/viewer/shared_state#tokio-%E3%81%AE%E9%9D%9E%E5%90%8C%E6%9C%9F-mutex-%E3%82%92%E5%88%A9%E7%94%A8%E3%81%99%E3%82%8B
//
// mpscはtokioのを使うべきだと思う...
// (演習側にこっちを持ってきたのを褒めるべきか？)
//
// どうブロッキングが起きるかの実験は以下に。(テストだと標準出力が終了まで見れない)
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=003766a72fd6c11fa17dd5d23ccca70f

pub struct Message {
    payload: String,
    response_channel: mpsc::Sender<Message>,
}

/// Replies with `pong` to any message it receives, setting up a new
/// channel to continue communicating with the caller.
pub async fn pong(mut receiver: mpsc::Receiver<Message>) {
    loop {
        if let Some(msg) = receiver.recv().await {
            println!("Pong received: {}", msg.payload);
            let (sender, new_receiver) = mpsc::channel(1);
            msg.response_channel
                .send(Message {
                    payload: "pong".into(),
                    response_channel: sender,
                })
                .await
                .unwrap();
            receiver = new_receiver;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{pong, Message};
    use tokio::sync::mpsc;

    // testは都合よくシングルスレッド
    #[tokio::test]
    async fn ping() {
        let (sender, receiver) = mpsc::channel(1);
        let (response_sender, mut response_receiver) = mpsc::channel(1);
        sender
            .send(Message {
                payload: "pong".into(),
                response_channel: response_sender,
            })
            .await
            .unwrap();

        tokio::spawn(pong(receiver));

        let answer = response_receiver.recv().await.unwrap().payload;
        assert_eq!(answer, "pong");
    }
}
