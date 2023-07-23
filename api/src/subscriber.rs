use super::client::Client;
use ethers::providers::Middleware;
use futures::Stream;
use std::{collections::VecDeque, pin::Pin, sync::Arc};
use tokio::sync::Mutex;
use tokio::sync::Notify;
use uniswapx_sdk_core::order::ValidationStatus;
use uniswapx_sdk_core::{
    order::Order,
    utils::{run_with_shutdown, spawn_with_shutdown, OrderCache},
};

#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn};

// todo make this a trait
// can have 'subscribe' and 'is_expired' as abstract methods
// it should have a `Self::Target`

// make we can make the buffer fun

pub struct OrderSubscriber;

impl OrderSubscriber {
    /// a never ending subscription to open orders
    ///
    /// this stream can return invalid orders, consumers are expected to validate ([Order].validate_ethers()) them before use as they can expire at anytime
    /// if the upstream [Client] returns invalid orders this stream will return them
    pub fn subscribe<C, M>(
        cache: Arc<OrderCache>,
        provider: Arc<M>,
        client: Arc<C>,
        sleep: u64,
    ) -> Pin<Box<impl Stream<Item = Order>>>
    where
        C: Client<Order> + 'static,
        M: Middleware + 'static,
    {
        let buf = Arc::new(Mutex::new(VecDeque::new()));
        let waker = Arc::new(tokio::sync::Notify::new());

        // spawn a task that dumps unseen orders into the buffer
        spawn_with_shutdown(Self::fill_buf(
            buf.clone(),
            cache.clone(),
            provider.clone(),
            client.clone(),
            waker.clone(),
            sleep,
        ));

        Box::pin(async_stream::stream! {
            // wait for the first buf fill
            waker.notified().await;
            while let Some(order) = run_with_shutdown(Self::read_buf(buf.clone(), waker.clone())).await {
                yield order;
            }
        })
    }

    /// awaits a notification from task filling buf iff no orders are in buf
    ///
    // inline async blocks dont seem to work in a stream! macro so we need this function
    async fn read_buf(buf: Arc<Mutex<VecDeque<Order>>>, waker: Arc<Notify>) -> Order {
        loop {
            let mut buf = buf.lock().await;

            match buf.pop_front() {
                Some(order) => {
                    return order;
                }
                None => {
                    drop(buf);
                    waker.notified().await;
                }
            }
        }
    }

    // hits the api and condintally pushes orders into the buffer
    // if the client returns expired orders they will be pushed into the buffer
    async fn fill_buf<C, M>(
        buf: Arc<Mutex<VecDeque<Order>>>,
        cache: Arc<OrderCache>,
        provider: Arc<M>,
        client: Arc<C>,
        waker: Arc<Notify>,
        sleep: u64,
    ) where
        C: Client<Order> + 'static,
        M: Middleware + 'static,
    {
        loop {
            let mut buf = buf.lock().await;
            let mut cache = cache.lock().await;

            let orders = client.firehose().await;

            // will try again so just continue
            if orders.is_err() {
                error!("subscriber: error getting order froms the client");
                continue;
            }

            let orders = orders.unwrap();

            // spawns a non blocking task to get debug metrics on the orders
            // could be bad casue the clone tho if theres tons of orders
            debug_validation(orders.clone(), provider.clone());

            debug!(
                "subsciber got orders: {:?}, buf size: {:?}",
                orders.len(),
                buf.len()
            );

            if orders.len() == 0 {
                continue;
            }

            let len_before = buf.len();

            // could filter map and extend
            for order in orders {
                if !cache.contains_key(&order.struct_hash().to_string()) {
                    cache.insert(order.struct_hash().to_string(), order.clone());
                    buf.push_back(order);
                }
            }

            let len_after = buf.len();

            drop(cache);
            drop(buf);

            if len_before == 0 && len_after > 0 {
                waker.notify_one();
            }

            tokio::time::sleep(std::time::Duration::from_secs(sleep)).await;
        }
    }
}

fn debug_validation<M: Middleware + 'static>(orders: Vec<Order>, provider: Arc<M>) {
    tokio::spawn(async move {
        debug!(
            "order validations: {:?}",
            futures::future::join_all(
                orders
                    .iter()
                    .map(|order| order.validate_ethers(provider.clone())),
            )
            .await
            .iter()
            .map(|res| match res {
                Ok(ans) => Some(ans),
                Err(e) => None,
            })
            .collect::<Vec<_>>()
        )
    });
}
