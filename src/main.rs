use crate::http_client::http_request;

mod http_client;
#[tokio::main]
async fn main() {
    println!("Hello, world!");

    http_request().await;
}
