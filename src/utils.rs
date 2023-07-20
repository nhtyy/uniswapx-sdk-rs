use tokio::{select, signal, spawn, task::JoinHandle};

pub fn spawn_with_shutdown<Fut, T>(future: Fut) -> JoinHandle<Option<T>>
where
    Fut: std::future::Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    spawn(async {
        select! {
            res = future => Some(res),
            _ = signal::ctrl_c() => {
                None
            }
        }
    })
}

pub async fn run_with_shutdown<Fut, T>(future: Fut) -> Option<T>
where
    Fut: std::future::Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    select! {
        ret = future => Some(ret),
        _ = signal::ctrl_c() => {
            None
        }
    }
}
