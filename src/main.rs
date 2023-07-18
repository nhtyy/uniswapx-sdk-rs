pub mod api;
pub mod contracts;
pub mod order;
pub mod server;
pub mod subscriber;

use api::uniswap::{ApiParams, OrderClient};
use order::Order;

#[tokio::main]
async fn main() {
    let client = OrderClient::new();

    let mut res = client
        .get_orders(ApiParams {
            limit: 10,
            chain_id: 1,
            order_status: api::uniswap::response_types::OrderStatus::Open,
        })
        .await
        .unwrap();

    println!("{:#?}", res);

    let order = Order::try_from(res.orders.pop().unwrap()).unwrap();
}
