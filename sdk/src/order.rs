use crate::contracts::internal::{
    common::OrderInfo, dutch::DutchOrder, exclusive_dutch::ExclusiveDutchOrder, limit::LimitOrder,
};
use alloy_sol_types::{sol_data::Uint, SolStruct, SolType, B256};

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

// todo macro
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

    pub fn order_type(&self) -> crate::api::OrderType {
        match self {
            OrderInner::Dutch(_) => crate::api::OrderType::Dutch,
            OrderInner::Limit(_) => crate::api::OrderType::Limit,
            OrderInner::ExclusiveDutch(_) => crate::api::OrderType::ExclusiveDutch,
        }
    }
}

// part of macro also
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

    pub fn order_type(&self) -> crate::api::OrderType {
        self.inner.order_type()
    }

    pub fn deadline(&self) -> <Uint<256> as SolType>::RustType {
        self.info().deadline
    }

    pub fn validate(&self) -> bool {
        /// needs bindings
        todo!()
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

impl TryFrom<String> for DutchOrder {
    type Error = alloy_sol_types::Error;

    fn try_from(hex_encoded: String) -> Result<Self, Self::Error> {
        DutchOrder::hex_decode_single(&hex_encoded, true)
    }
}

impl TryFrom<String> for LimitOrder {
    type Error = alloy_sol_types::Error;

    fn try_from(hex_encoded: String) -> Result<Self, Self::Error> {
        LimitOrder::hex_decode_single(&hex_encoded, true)
    }
}

impl TryFrom<String> for ExclusiveDutchOrder {
    type Error = alloy_sol_types::Error;

    fn try_from(hex_encoded: String) -> Result<Self, Self::Error> {
        ExclusiveDutchOrder::hex_decode_single(&hex_encoded, true)
    }
}
