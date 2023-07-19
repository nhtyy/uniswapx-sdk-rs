pub mod uniswap;
use crate::order::Order;
use serde::Deserialize;
use std::{error::Error, future::Future, pin::Pin};

/// https://github.com/Uniswap/uniswapx-sdk/blob/main/src/constants.ts
/// only used for deriving our types from external api calls
#[derive(Deserialize, Debug)]
pub enum OrderType {
    Dutch,
    Limit,
    ExclusiveDutch,
}

pub trait OrderClient {
    type ClientError: Error;

    /// should return as many open orders as possible
    /// should be up to the consumer to decide what to do with them
    fn get_open_orders(
        self,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Order>, Self::ClientError>>>>;
}
