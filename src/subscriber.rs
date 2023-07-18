use crate::order::Order;
use futures::stream::Stream;
use std::error::Error;

pub trait OrderSubscriber {
    type InternalErr: Error;

    fn subscribe(self) -> Box<dyn Stream<Item = Result<Order, Self::InternalErr>>>;
}
