use crate::order::Order;

#[async_trait::async_trait]
pub trait OrderClient: Send + Sync {
    type ClientError: std::error::Error;

    /// should return as many open orders as possible
    /// should be up to the consumer to decide what to do with them
    /// these
    async fn get_open_orders(&self) -> Result<Vec<Order>, Self::ClientError>;
}
