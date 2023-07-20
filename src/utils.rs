use crate::order::Order;
use alloy_sol_types::B256;
use futures::{Stream, StreamExt};
use std::{collections::HashMap, pin::Pin};
use tokio::{select, signal, spawn, task::JoinHandle};

pub fn spawn_with_shutdown<Fut, T>(future: Fut) -> JoinHandle<Option<T>>
where
    Fut: std::future::Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    spawn(run_with_shutdown(future))
}

pub async fn run_with_shutdown<Fut, T>(future: Fut) -> Option<T>
where
    Fut: std::future::Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    select! {
        ret = future => Some(ret),
        _ = signal::ctrl_c() => {
            println!("shutting down");
            None
        }
    }
}

/// convience struct for spawning a handler task
///
/// see [OrderHandler::spawn] for more info
pub struct OrderHandler {
    pub handle: JoinHandle<Option<()>>,
}

impl OrderHandler {
    /// spawn a handler task for a stream of orders
    ///
    /// this stream is expected not to end
    /// the handler doees not return a value
    pub fn spawn<S, Func>(mut stream: Pin<Box<S>>, mut handler: Func) -> Self
    where
        S: Stream<Item = Order> + Send + 'static,
        Func: FnMut(Order) -> () + Send + 'static,
    {
        let handle = spawn_with_shutdown(async move {
            loop {
                let order = stream.next().await.expect("this stream should never end");
                handler(order);
            }
        });

        Self { handle }
    }
}

pub struct OrderCache {
    cache: HashMap<String, Order>,
}

impl std::ops::Deref for OrderCache {
    type Target = HashMap<String, Order>;

    fn deref(&self) -> &Self::Target {
        &self.cache
    }
}

impl std::ops::DerefMut for OrderCache {
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

    pub fn flush_closed_orders(&mut self, timestamp: u64) {}
}
