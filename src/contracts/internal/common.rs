use ethers::{
    contract::{EthAbiCodec, EthAbiType},
    types::{Address, Bytes, U256},
};

// https://github.com/Uniswap/UniswapX/blob/7494d01d2efa7ef16aa3c4065e3fbd7db57c580c/src/base/ReactorStructs.sol#L10
#[derive(Clone, EthAbiType, EthAbiCodec, Debug, PartialEq, Eq, Hash)]
pub struct OrderInfo {
    reactor: Address,
    swapper: Address,
    nonce: U256,
    deadline: U256,
    additional_validation_contract: Address,
    additional_validation_data: Bytes,
}

#[derive(Clone, EthAbiType, EthAbiCodec, Debug, PartialEq, Eq, Hash)]
pub struct InputToken {
    token: Address,
    amount: U256,
    max_amount: U256,
}

#[derive(Clone, EthAbiType, EthAbiCodec, Debug, PartialEq, Eq, Hash)]
pub struct OutputToken {
    token: Address,
    amount: U256,
    recipient: Address,
}

#[derive(Clone, EthAbiType, EthAbiCodec, Debug, PartialEq, Eq, Hash)]
pub struct ResolvedOrder {
    pub info: OrderInfo,
    pub input: InputToken,
    pub outputs: Vec<OutputToken>,
}

/// used in the contract as entrypoint to all reactors
#[derive(Clone, EthAbiType, EthAbiCodec, Debug, PartialEq, Eq, Hash)]
pub struct SignedOrder {
    order: Bytes,
    sig: Bytes,
}
