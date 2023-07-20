pub mod api;
pub mod contracts;
pub mod order;
pub mod server;

use api::{client::OrderClient, subscriber::OrderSubscriber, uniswap::UniswapClient};
use futures::stream::StreamExt;
use futures_util::pin_mut;
use order::Order;

#[tokio::main]
async fn main() {
    let client = std::sync::Arc::new(UniswapClient::new(1));

    let mut stream = OrderSubscriber::subscribe(client, 5);

    while let Some(order) = stream.next().await {
        match order {
            Ok(order) => {
                println!("got order: {:?}", order.info().deadline);
            }
            Err(e) => {
                println!("error: {:?}", e);
            }
        }
    }
}
