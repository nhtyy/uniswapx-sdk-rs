use super::client::OrderClient;
use crate::order::Order;
use futures::Stream;
use std::{collections::VecDeque, sync::Arc};
use tokio::sync::Mutex;
use tokio::sync::Notify;

pub struct OrderSubscriber {}

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
    loop {
        let mut buf = buf.lock().await;

        match client.get_open_orders().await {
            Ok(orders) => {
                println!("buf: {}, orders: {}", buf.len(), orders.len());

                let len_before = buf.len();
                for order in orders {
                    // if !buf.contains(&order) { // todo! partial eq
                    //     buf.push_back(order);
                    // }

                    buf.push_back(order);
                }

                if len_before == 0 && buf.len() > 0 {
                    waker.notify_one();
                }
            }
            Err(e) => {
                println!("error: {:?}", e);
            }
        }

        drop(buf);

        tokio::time::sleep(std::time::Duration::from_secs(sleep)).await;
    }
}
