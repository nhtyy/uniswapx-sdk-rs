use super::client::OrderClient;
use crate::order::Order;
use alloy_sol_types::SolType;
use alloy_sol_types::B256;
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
