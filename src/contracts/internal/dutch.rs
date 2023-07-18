use super::common::OrderInfo;
use ethers::{
    contract::{EthAbiCodec, EthAbiType},
    types::{Address, U256},
};

#[derive(Clone, EthAbiType, EthAbiCodec, Debug, PartialEq, Eq, Hash)]
pub struct DutchOrder {
    pub info: OrderInfo,
    pub decay_start_time: U256,
    pub decay_end_time: U256,
    pub input: DutchInput,
    pub output: Vec<DutchOutput>,
}

#[derive(Clone, EthAbiType, EthAbiCodec, Debug, PartialEq, Eq, Hash)]
pub struct DutchOutput {
    token: Address,
    start_amount: U256,
    end_amount: U256,
    recipient: Address,
}

#[derive(Clone, EthAbiType, EthAbiCodec, Debug, PartialEq, Eq, Hash)]
pub struct DutchInput {
    token: Address,
    start_amount: U256,
    end_amount: U256,
}
