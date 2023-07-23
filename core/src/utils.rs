use crate::order::{Order, ValidationStatus};
use ethers::providers::Middleware;
use std::{collections::HashMap, pin::Pin, sync::Arc};
use tokio::{select, signal, spawn, sync::Mutex, task::JoinHandle};

#[allow(unused_imports)]
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
            info!("ctrl-c received, shutting down");
            None
        }
    }
}

/// a task safe cache meant to be shared across subscribers
/// it is instanitaed with a [tokio::task] to flush its cache periodically
pub struct OrderCache {
    cache: Mutex<HashMap<String, Order>>,
}

impl std::ops::Deref for OrderCache {
    type Target = Mutex<HashMap<String, Order>>;

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
    /// spawns a task that flushes the cache every `flush_interval` seconds
    ///
    /// uses the [Middleware] to validate orders
    pub fn new<M: Middleware + 'static>(provider: Arc<M>, flush_interval: u64) -> Arc<Self> {
        let new = Arc::new(Self {
            cache: Mutex::new(HashMap::new()),
        });

        Self::spawn_flush_task(new.clone(), provider, flush_interval);

        new
    }

    /// flushes the cache, removing invalid orders
    pub async fn flush<M>(self: Arc<Self>, provider: std::sync::Arc<M>)
    where
        M: Middleware + 'static,
    {
        let mut lock = self.cache.lock().await;

        let (keys, futures): (Vec<_>, Vec<_>) = lock
            .iter()
            .map(|(k, order)| (k.clone(), order.validate_ethers(provider.clone())))
            .unzip();

        let results = futures::future::join_all(futures).await;

        for (key, result) in keys.into_iter().zip(results.into_iter()) {
            match result {
                Ok(ValidationStatus::OK) => {
                    info!("order {} is valid, keeping", key);
                }
                Ok(_) => {
                    info!("order {} is invalid, removing", key);
                    lock.remove(&key);
                }
                Err(e) => {
                    error!(
                        "error validating order when flushing cache {}: {:?}",
                        key, e
                    );
                }
            }
        }

        // explicity drop lock for sanity
        drop(lock);
    }

    // doesnt return a join handle but is spawned with shutdown
    fn spawn_flush_task<M>(self: Arc<Self>, provider: Arc<M>, flush_interval: u64)
    where
        M: Middleware + 'static,
    {
        spawn_with_shutdown(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(flush_interval)).await;
                self.clone().flush(provider.clone()).await;
            }
        });
    }
}
