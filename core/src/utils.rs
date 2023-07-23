use crate::order::{Order, ValidationStatus};
use ethers::providers::Middleware;
use futures::{Stream, StreamExt};
use std::{collections::HashMap, pin::Pin, sync::Arc};
use tokio::{
    select, signal, spawn,
    sync::{Mutex, MutexGuard},
    task::JoinHandle,
};

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
            debug!("ctrl-c received, shutting down");
            None
        }
    }
}

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
    pub fn new() -> Self {
        Self {
            cache: Mutex::new(HashMap::new()),
        }
    }

    pub async fn flush<M>(
        map: &mut MutexGuard<'_, HashMap<String, Order>>,
        provider: std::sync::Arc<M>,
    ) where
        M: Middleware + 'static,
    {
        let (keys, futures): (Vec<_>, Vec<_>) = map
            .iter()
            .map(|(k, order)| (k.clone(), order.validate_ethers(provider.clone())))
            .unzip();

        let results = futures::future::join_all(futures).await;

        for (key, result) in keys.into_iter().zip(results.into_iter()) {
            match result {
                Ok(ValidationStatus::OK) => {
                    debug!("order {} is valid, keeping", key);
                }
                Ok(_) => {
                    debug!("order {} is invalid, removing", key);
                    map.remove(&key);
                }
                Err(e) => {
                    error!(
                        "error validating order when flushing cache {}: {:?}",
                        key, e
                    );
                }
            }
        }
    }
}
