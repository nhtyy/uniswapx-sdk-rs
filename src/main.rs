pub mod api;
pub mod contracts;
pub mod order;
pub mod server;

use api::{uniswap::UniswapClient, OrderClient};
use order::Order;

#[tokio::main]
async fn main() {
    let client = UniswapClient::new(1);

    let mut res = client.get_open_orders().await.unwrap();

    for order in res.iter() {
        println!("{:?}", order.info().deadline.to_string());
    }
}
