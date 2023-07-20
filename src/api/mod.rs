pub mod client;
pub mod subscriber;
pub mod uniswap;
use serde::Deserialize;

/// https://github.com/Uniswap/uniswapx-sdk/blob/main/src/constants.ts
/// only used for deriving our types from external api calls
#[derive(Deserialize, Debug)]
pub enum OrderType {
    Dutch,
    Limit,
    ExclusiveDutch,
}

pub async fn with_shutdown<Fut, T>(future: Fut) -> Option<T>
where
    Fut: std::future::Future<Output = T>,
{
    tokio::select! {
        res = future => Some(res),
        _ = tokio::signal::ctrl_c() => {
            println!("ctrl-c received, exiting");
            None
        }
    }
}
