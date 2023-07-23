A crate for consuming, and coordinating filler services for UniswapX orders.

see `cargo book --open` to see the full docs

The crate will have 3 main parts, api, server and core.

- api:
  - subscribers and clients are defined here. There is a default uniswap client built in.
  - you can wrap any api by implementing the async trait `Client<Order>` over it
- core:
  - Alloy-rs structs live here
  - contains validations in the style of the UniswapX-sdk in javascript
  - an order cache that can be shared between subscribers
  - order builders (coming soon)
- server (coming soon)
  - A tokio friendly UniswapX order api framework

An example setup

```rust
use ethers::providers::{Http, Middleware};
use futures::StreamExt;
use std::sync::Arc;
use tokio::sync::Mutex;
use uniswapx_sdk_api::{subscriber::OrderSubscriber, uniswap::UniswapClient};
use uniswapx_sdk_core::{order::Order, utils::OrderCache};

const PROVIDER_URL: &str = "";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let provider = Arc::new(
        ethers::providers::Provider::<Http>::try_from(PROVIDER_URL).expect("provider url to parse"),
    );

    let client = Arc::new(UniswapClient::new(1));

    let cache = Arc::new(OrderCache::new());

    let mut sub = OrderSubscriber::subscribe(cache, provider.clone(), client, 5);

    while let Some(order) = sub.next().await {
        tokio::spawn(handle_order(order, provider.clone()));
    }
}

async fn handle_order<M: Middleware + 'static>(order: Order, provider: Arc<M>) {
    println!("reactor: {:?}", order.reactor_address());

    match order.validate_ethers(provider.clone()).await {
        Ok(ans) => {
            println!("Isvalid? {:?}", ans);
            println!("deadline: {:?}", order.deadline());

            // do stuff with order
        }
        Err(e) => {
            println!("error: {:?}", e);
        }
    }
}


```