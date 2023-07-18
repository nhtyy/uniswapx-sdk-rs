use super::common::{InputToken, OrderInfo, OutputToken};
use ethers::contract::{EthAbiCodec, EthAbiType};

#[derive(Clone, EthAbiType, EthAbiCodec, Debug, PartialEq, Eq, Hash)]
pub struct LimitOrder {
    info: OrderInfo,
    input: InputToken,
    outputs: Vec<OutputToken>,
}
