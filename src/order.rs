use crate::contracts::internal::{
    dutch::DutchOrder, exclusive_dutch::ExclusiveDutchOrder, limit::LimitOrder,
};

use ethers::types::Bytes;

/// internal type we use to represnt orders
/// this should use ethers types
#[derive(Debug)]
pub struct Order {
    inner: OrderInner,
    sig: Bytes,
}

impl Order {
    pub fn new(inner: OrderInner, sig: Bytes) -> Self {
        Self { inner, sig }
    }
}

#[derive(Debug)]
pub enum OrderInner {
    Dutch(DutchOrder),
    Limit(LimitOrder),
    ExclusiveDutch(ExclusiveDutchOrder),
}

impl From<DutchOrder> for OrderInner {
    fn from(order: DutchOrder) -> Self {
        OrderInner::Dutch(order)
    }
}

impl From<LimitOrder> for OrderInner {
    fn from(order: LimitOrder) -> Self {
        OrderInner::Limit(order)
    }
}

impl From<ExclusiveDutchOrder> for OrderInner {
    fn from(order: ExclusiveDutchOrder) -> Self {
        OrderInner::ExclusiveDutch(order)
    }
}

impl Order {
    pub fn validate(&self) -> bool {
        todo!()
    }
}
