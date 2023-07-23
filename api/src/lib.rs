/// the [Client] trait, an [async_trait::async_trait] for an abstract client over some type of endpoint
pub mod client;

/// implementaion of [OrderSubscriber] of [Client<Order>]
pub mod subscriber;

/// a default implemantion of [Client<Order>] around the original uniswapX api
pub mod uniswap;
