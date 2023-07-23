use uniswapx_sdk_core::order::Order;

#[async_trait::async_trait]
pub trait Client<T>: Send + Sync {
    type ClientError: std::error::Error + Send + Sync + 'static;

    /// should return as many open orders as possible
    async fn firehose(&self) -> Result<Vec<T>, Self::ClientError>;
}
