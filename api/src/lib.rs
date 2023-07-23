/// the [Client] trait, an [async_trait::async_trait] for an abstract client over some type of endpoint
pub mod client;

/// a default implemantion of [Client<Order>] around the original uniswapX api
pub mod uniswap;

/// implementaion of [OrderSubscriber] which wraps a [Client<Order>]
pub mod subscriber;
