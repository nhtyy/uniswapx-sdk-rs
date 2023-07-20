use super::client::OrderClient;
use crate::{
    order::Order,
    utils::{run_with_shutdown, spawn_with_shutdown},
};
use alloy_sol_types::B256;
use futures::Stream;
use std::{
    collections::{HashMap, VecDeque},
    ops::{Deref, DerefMut},
    pin::Pin,
    sync::Arc,
};
use tokio::sync::Mutex;
use tokio::sync::Notify;

type OrderStream<C: OrderClient> = Pin<Box<dyn Stream<Item = Result<Order, C::ClientError>>>>;

pub struct OrderSubscriber;

pub struct OrderCache {
    cache: HashMap<B256, Order>,
}

impl Deref for OrderCache {
    type Target = HashMap<B256, Order>;

    fn deref(&self) -> &Self::Target {
        &self.cache
    }
}

impl DerefMut for OrderCache {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cache
    }
}

impl OrderCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub fn flush_closed_orders(&mut self) {
        todo!()
    }
}

/// a never ending subscription to open orders
impl OrderSubscriber {
    /// warning: assumes respsones will always be in the same order!
    pub fn subscribe<C: OrderClient + 'static>(
        client: Arc<C>,
        cache: Arc<Mutex<OrderCache>>,
        sleep: u64,
    ) -> OrderStream<C> {
        let buf = Arc::new(Mutex::new(VecDeque::new()));
        let waker = Arc::new(tokio::sync::Notify::new());

        // spawn a task that dumps unseen orders into the buffer
        // warning: assumes respsones will always be in the same order!
        spawn_with_shutdown(order_client_listener(
            buf.clone(),
            client.clone(),
            waker.clone(),
            sleep,
        ));

        // buf and waker get moved into here and i think basically get spawned as a task?
        Box::pin(async_stream::stream! {
            waker.notified().await;
            while let Some(order) = run_with_shutdown(Self::await_next(buf.clone(), waker.clone())).await {
                yield Ok(order);
            }
        })
    }

    /// awaits a notification from task filling buf iff no orders are in buf
    ///
    // inline async blocks dont seem to work in a stream! macro so we need this function
    async fn await_next(buf: Arc<Mutex<VecDeque<Order>>>, waker: Arc<Notify>) -> Order {
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
}

// warning: assumes respsones will always be in the same order!
// hits the api and condintally pushes them into a buffer
// todo broken! need cache
async fn order_client_listener<C: OrderClient>(
    buf: Arc<Mutex<VecDeque<Order>>>,
    client: Arc<C>,
    waker: Arc<Notify>,
    sleep: u64,
) {
    let mut last_hash: Option<B256> = None;

    loop {
        let mut buf = buf.lock().await;

        match client.get_open_orders().await {
            Ok(orders) => {
                if orders.len() == 0 {
                    println!("no orders received from client");
                    continue;
                }

                println!("buf: {}, orders: {}", buf.len(), orders.len());

                let len_before = buf.len();

                match last_hash {
                    Some(ref mut hash) => {
                        let maybe_pos = orders.iter().position(|o| o.inner.struct_hash() == *hash);

                        *hash = orders
                            .as_slice()
                            .last()
                            .map(|o| o.inner.struct_hash())
                            .expect("should have a last hash");

                        match maybe_pos {
                            Some(pos) => buf.extend(orders.into_iter().skip(pos + 1)),
                            None => buf.extend(orders),
                        }
                    }
                    None => {
                        last_hash = Some(
                            orders
                                .as_slice()
                                .last()
                                .map(|o| o.inner.struct_hash())
                                .expect("should have a last hash"),
                        );

                        buf.extend(orders);
                    } // todo! here we should set the last hash
                }

                drop(buf);

                if len_before == 0 {
                    waker.notify_one();
                }
            }
            Err(e) => {
                println!("error: {:?}", e);
            }
        }

        tokio::time::sleep(std::time::Duration::from_secs(sleep)).await;
    }
}
