use super::{
    common::OrderInfo,
    dutch::{DutchInput, DutchOutput},
};
use ethers::{
    contract::{EthAbiCodec, EthAbiType},
    types::{Address, U256},
};

#[derive(Clone, EthAbiType, EthAbiCodec, Debug, PartialEq, Eq, Hash)]
pub struct ExclusiveDutchOrder {
    info: OrderInfo,
    decay_start_time: U256,
    decay_end_time: U256,
    exclusive_filler: Address,
    exclusivity_override_bps: U256,
    input: DutchInput,
    output: Vec<DutchOutput>,
}
