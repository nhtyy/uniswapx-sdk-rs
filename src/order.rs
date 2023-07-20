use crate::{
    contracts::internal::{
        common::{OrderInfo, SignedOrder},
        dutch::DutchOrder,
        exclusive_dutch::ExclusiveDutchOrder,
        limit::LimitOrder,
    },
    utils::spawn_with_shutdown,
};
use alloy_sol_types::{sol_data::Uint, SolStruct, SolType, B256};
use futures::{Stream, StreamExt};
use std::pin::Pin;
use tokio::task::JoinHandle;

#[derive(Clone)]
pub struct Order {
    pub inner: OrderInner,
    pub sig: String,
}

#[derive(Clone)]
pub enum OrderInner {
    Dutch(DutchOrder),
    Limit(LimitOrder),
    ExclusiveDutch(ExclusiveDutchOrder),
}

impl OrderInner {
    pub fn info(&self) -> &OrderInfo {
        match self {
            OrderInner::Dutch(o) => &o.info,
            OrderInner::Limit(o) => &o.info,
            OrderInner::ExclusiveDutch(o) => &o.info,
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        match self {
            OrderInner::Dutch(o) => DutchOrder::encode(o),
            OrderInner::Limit(o) => LimitOrder::encode(o),
            OrderInner::ExclusiveDutch(o) => ExclusiveDutchOrder::encode(o),
        }
    }

    pub fn struct_hash(&self) -> B256 {
        match self {
            OrderInner::Dutch(o) => o.eip712_hash_struct(),
            OrderInner::Limit(o) => o.eip712_hash_struct(),
            OrderInner::ExclusiveDutch(o) => o.eip712_hash_struct(),
        }
    }

    pub fn type_hash(&self) -> B256 {
        match self {
            OrderInner::Dutch(o) => o.eip712_type_hash(),
            OrderInner::Limit(o) => o.eip712_type_hash(),
            OrderInner::ExclusiveDutch(o) => o.eip712_type_hash(),
        }
    }
}

impl Order {
    pub fn new(inner: OrderInner, sig: String) -> Self {
        Self { inner, sig }
    }

    pub fn info(&self) -> &OrderInfo {
        self.inner.info()
    }

    pub fn struct_hash(&self) -> B256 {
        self.inner.struct_hash()
    }

    pub fn type_hash(&self) -> B256 {
        self.inner.type_hash()
    }

    pub fn deadline(&self) -> <Uint<256> as SolType>::RustType {
        self.info().deadline
    }

    pub fn validate(&self) -> bool {
        /// needs bindings
        todo!()
    }
}

pub struct OrderHandler {
    pub handle: JoinHandle<Option<()>>,
}

impl OrderHandler {
    /// spawn a handler for a stream of orders
    ///
    /// this stream is expected not to end
    pub fn spawn<S, Func>(mut stream: Pin<Box<S>>, mut handler: Func) -> Self
    where
        S: Stream<Item = Order> + Send + 'static,
        Func: FnMut(Order) -> () + Send + 'static,
    {
        let handle = spawn_with_shutdown(async move {
            loop {
                let order = stream.next().await.expect("this stream should never end");
                handler(order);
            }
        });

        Self { handle }
    }
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

impl From<Order> for SignedOrder {
    fn from(order: Order) -> Self {
        // Self {
        //     order: order.inner.encode(),
        //     sig: todo!(), // hex decode into bytes, then make a bytes type
        // }

        todo!()
    }
}
