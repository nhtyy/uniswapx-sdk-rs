pub mod uniswap;
use crate::order::Order;
use futures::Stream;
use serde::Deserialize;
use std::{collections::VecDeque, error::Error, future::Future, pin::Pin};

/// https://github.com/Uniswap/uniswapx-sdk/blob/main/src/constants.ts
/// only used for deriving our types from external api calls
#[derive(Deserialize, Debug)]
pub enum OrderType {
    Dutch,
    Limit,
    ExclusiveDutch,
}

pub struct OrderSubscriber<C: OrderClient> {
    cache: VecDeque<Order>,
    idx: usize,

    client: C,
}

/// a never ending subscription to open orders
impl<C: OrderClient> OrderSubscriber<C> {
    pub fn new(client: C) -> Self {
        Self {
            cache: VecDeque::new(),
            idx: 0,
            client,
        }

        // we could spawn a task here to dump into cache
        // maybe use waker model to signal when cache is updated
    }

    pub async fn next(&mut self) -> Order {
        todo!()
    }

    pub fn subscribe(mut self) -> impl Stream<Item = Order> {
        async_stream::stream! {
            yield self.next().await
        }
    }
}

pub trait OrderClient {
    type ClientError: Error;

    /// should return as many open orders as possible
    /// should be up to the consumer to decide what to do with them
    fn get_open_orders(
        self,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Order>, Self::ClientError>>>>;
}
