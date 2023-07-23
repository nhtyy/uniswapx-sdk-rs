#[async_trait::async_trait]
pub trait Client<T>: Send + Sync {
    type ClientError: std::error::Error + Send + Sync + 'static;

    /// dump of as many things as possible
    async fn firehose(&self) -> Result<Vec<T>, Self::ClientError>;
}
