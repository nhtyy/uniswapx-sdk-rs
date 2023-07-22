use uniswapx_sdk_core::order::Order;

#[async_trait::async_trait]
pub trait OrderClient: Send + Sync {
    type ClientError: std::error::Error + Send + Sync + 'static;

    /// should return as many open orders as possible
    async fn firehose(&self) -> Result<Vec<Order>, Self::ClientError>;
}
