use async_std::net::TcpListener;
use async_std::task::spawn;
use async_test::handle_connection;
use futures::stream::StreamExt;

#[async_std::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();

    // Use concorrency
    // listener.incoming().for_each_concurrent(None, |tcpstream| async move {
    //     let tcpstream = tcpstream.unwrap();
    //     handle_connection(tcpstream).await;
    // }).await;

    // use both concorrency and parallelism
    listener
        .incoming()
        .for_each_concurrent(None, |stream| async move {
            let stream = stream.unwrap();
            spawn(handle_connection(stream));
        })
        .await;
}
