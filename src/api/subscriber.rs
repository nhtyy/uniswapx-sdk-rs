use super::client::OrderClient;
use crate::order::Order;
use futures::Stream;
use std::{collections::VecDeque, sync::Arc};
use tokio::sync::Mutex;
use tokio::sync::Notify;

pub struct OrderSubscriber;

/// a never ending subscription to open orders
impl OrderSubscriber {
    pub fn subscribe<C: OrderClient + 'static>(
        client: Arc<C>,
        sleep: u64,
    ) -> impl Stream<Item = Result<Order, C::ClientError>> {
        let buf: Arc<Mutex<VecDeque<Order>>> = Arc::new(Mutex::new(VecDeque::new()));
        let waker = Arc::new(tokio::sync::Notify::new());

        tokio::spawn(order_client_listener(
            buf.clone(),
            client.clone(),
            waker.clone(),
            sleep,
        ));

        async_stream::stream! {
            waker.notified().await;
            loop {
                tokio::select! {
                    order = async {
                        loop {
                            let mut buf = buf.lock().await;
                            match buf.pop_front() {
                                Some(order) => {
                                    return Ok(order);
                                }
                                None => {
                                    drop(buf);
                                    waker.notified().await;
                                }
                            }
                        }
                    } => {
                        yield order;
                    },
                    _ = tokio::signal::ctrl_c() => {
                        println!("shutting down");
                        return;
                    }
                }
            }
        }
    }
}

async fn order_client_listener<C: OrderClient>(
    buf: Arc<Mutex<VecDeque<Order>>>,
    client: Arc<C>,
    waker: Arc<Notify>,
    sleep: u64,
) {
    let mut last_hash: Option<String> = None;

    loop {
        let mut buf = buf.lock().await;

        match client.get_open_orders().await {
            Ok(orders) => {
                println!("buf: {}, orders: {}", buf.len(), orders.len());

                let len_before = buf.len();

                match last_hash {
                    Some(ref mut hash) => {} // todo! here we should filter out orders we already have
                    None => buf.extend(orders), // todo! here we should set the last hash
                }

                let len_after = buf.len();

                drop(buf);

                if len_before == 0 && len_after > 0 {
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
