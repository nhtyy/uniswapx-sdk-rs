use crate::order::Order;
use ethers::providers::Middleware;
use futures::{Stream, StreamExt};
use std::{collections::HashMap, pin::Pin};
use tokio::{select, signal, spawn, task::JoinHandle};
use tracing::{debug, error, info, trace, warn};

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
            debug!("ctrl-c received, shutting down");
            None
        }
    }
}

/// convience struct for spawning a handler task
///
/// see [OrderHandler::spawn] for more info
pub struct TaskHandler {
    pub handle: JoinHandle<Option<()>>,
}

impl TaskHandler {
    /// spawn a handler task for a stream
    ///    /// the handler doees not return a value
    pub fn spawn<S, I, Func>(mut stream: Pin<Box<S>>, mut handler: Func) -> Self
    where
        S: Stream<Item = I> + Send + 'static,
        Func: FnMut(I) -> () + Send + 'static,
    {
        let handle = spawn_with_shutdown(async move {
            loop {
                if let Some(item) = stream.next().await {
                    handler(item);
                } else {
                    debug!("task handler steram returned none shutting");
                    break;
                }
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

    pub async fn flush_closed_orders<M: Middleware + 'static>(
        &mut self,
        provider: std::sync::Arc<M>,
    ) {
        let (keys, futures): (Vec<_>, Vec<_>) = self
            .iter()
            .map(|(k, order)| (k.clone(), order.validate_ethers(provider.clone())))
            .unzip();

        let results = futures::future::join_all(futures).await;

        for (key, result) in keys.into_iter().zip(results.into_iter()) {
            match result {
                Ok(false) => {
                    debug!("order {} is invalid, removing", key);
                    self.remove(&key);
                }
                Ok(_) => {
                    debug!("order {} is valid, keeping", key);
                }
                Err(e) => {
                    error!(
                        "error validating order when flushing subscriber cache {}: {:?}",
                        key, e
                    );
                }
            }
        }
    }
}
