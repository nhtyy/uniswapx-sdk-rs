use crate::contracts::internal::{
    common::{OrderInfo, ResolvedOrder},
    dutch::DutchOrder,
    exclusive_dutch::ExclusiveDutchOrder,
    limit::LimitOrder,
};
use alloy_primitives::{Address, B256, U256};
use alloy_sol_types::{SolStruct, SolType};

use ethers::{
    abi::AbiEncode,
    contract::ContractError,
    prelude::ContractCall,
    providers::Middleware,
    types::{Address as EthersAddress, ParseBytesError},
};
use serde::Deserialize;
use std::sync::Arc;
use uniswapx_ethers_bindings::order_quoter::order_quoter::{
    OrderQuoter, ResolvedOrder as EthersResolvedOrder,
};

/// https://github.com/Uniswap/uniswapx-sdk/blob/main/src/constants.ts
/// only used for deriving our types from external api calls
#[derive(Deserialize, Debug)]
pub enum OrderType {
    Dutch,
    Limit,
    ExclusiveDutch,
}

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
            OrderInner::Dutch(o) => DutchOrder::encode_single(o),
            OrderInner::Limit(o) => LimitOrder::encode_single(o),
            OrderInner::ExclusiveDutch(o) => ExclusiveDutchOrder::encode_single(o),
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

    pub fn order_type(&self) -> OrderType {
        match self {
            OrderInner::Dutch(_) => OrderType::Dutch,
            OrderInner::Limit(_) => OrderType::Limit,
            OrderInner::ExclusiveDutch(_) => OrderType::ExclusiveDutch,
        }
    }
}

#[derive(Debug)]
pub enum ValidationError<M: Middleware> {
    ContractError(ContractError<M>),
    SigParseError(ParseBytesError),
}

#[derive(Debug)]
pub enum ValidationStatus {
    Expired,
    NonceUsed,
    InsufficientFunds,
    InvalidSignature,
    InvalidOrderFields,
    UnknownError,
    ValidationFailed,
    ExclusivityPeriod,
    OK,
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

    pub fn order_type(&self) -> OrderType {
        self.inner.order_type()
    }

    pub fn deadline(&self) -> U256 {
        self.info().deadline
    }

    pub fn encode(&self) -> Vec<u8> {
        self.inner.encode()
    }

    pub fn reactor_address(&self) -> Address {
        self.info().reactor
    }

    pub fn quoter_address(&self) -> Address {
        // todo should be a mapping, but api only supports one order type rn
        "0x7714520f383C998e8822E8743FD6f90A2979689b"
            .parse()
            .expect("quoter address to parse")
    }

    pub async fn validate_ethers<M: Middleware + 'static>(
        &self,
        middleware: Arc<M>,
    ) -> Result<bool, ValidationError<M>> {
        match self.quote_ethers(middleware, self.quoter_address()).await {
            Ok(_) => Ok(true),
            Err(ValidationError::ContractError(ContractError::Revert(bytes))) => {
                println!("revert bytes: {:?}", bytes);
                Ok(false)
            }
            Err(err) => Err(err),
        }
    }

    pub async fn quote_ethers<M: Middleware + 'static>(
        &self,
        middleware: Arc<M>,
        quoter_address: Address,
    ) -> Result<ResolvedOrder, ValidationError<M>> {
        Ok(into_alloy_resolved_order(
            self.quote_contract_call(
                middleware,
                quoter_address
                    .to_string()
                    .parse::<EthersAddress>()
                    .expect("alloy type to parse by ethers"),
            )?
            .await?,
        ))
    }

    fn quote_contract_call<M: Middleware>(
        &self,
        middleware: Arc<M>,
        quoter_address: EthersAddress,
    ) -> Result<ContractCall<M, EthersResolvedOrder>, ParseBytesError> {
        let quoter = OrderQuoter::new(quoter_address, middleware);

        Ok(quoter.quote(self.encode().into(), self.sig.parse()?))
    }
}

fn into_alloy_resolved_order(ethers: EthersResolvedOrder) -> ResolvedOrder {
    ResolvedOrder::decode_single(&ethers.encode(), true)
        .expect("for ethers abi encoding to parse into an alloys resolved order")
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

impl<M: Middleware> std::error::Error for ValidationError<M> {}

impl<M: Middleware> std::fmt::Display for ValidationError<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::ContractError(e) => write!(f, "ContractError: {}", e),
            ValidationError::SigParseError(e) => write!(f, "SigParseError: {}", e),
        }
    }
}

impl<M: Middleware> From<ContractError<M>> for ValidationError<M> {
    fn from(e: ContractError<M>) -> Self {
        ValidationError::ContractError(e)
    }
}

impl<M: Middleware> From<ParseBytesError> for ValidationError<M> {
    fn from(e: ParseBytesError) -> Self {
        ValidationError::SigParseError(e)
    }
}
