pub mod api;
pub mod contracts;
pub mod order;
pub mod server;
pub mod utils;

use api::{
    subscriber::{OrderCache, OrderSubscriber},
    uniswap::UniswapClient,
};
use futures::stream::StreamExt;
use std::sync::Arc;

use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let client = std::sync::Arc::new(UniswapClient::new(1));
    let cache = Arc::new(Mutex::new(OrderCache::new()));

    let mut stream = OrderSubscriber::subscribe(cache, client, 5);

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
