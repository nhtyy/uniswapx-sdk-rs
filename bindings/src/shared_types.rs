///`DutchInput(address,uint256,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct DutchInput {
    pub token: ::ethers::core::types::Address,
    pub start_amount: ::ethers::core::types::U256,
    pub end_amount: ::ethers::core::types::U256,
}
///`DutchOutput(address,uint256,uint256,address)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct DutchOutput {
    pub token: ::ethers::core::types::Address,
    pub start_amount: ::ethers::core::types::U256,
    pub end_amount: ::ethers::core::types::U256,
    pub recipient: ::ethers::core::types::Address,
}
///`InputToken(address,uint256,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct InputToken {
    pub token: ::ethers::core::types::Address,
    pub amount: ::ethers::core::types::U256,
    pub max_amount: ::ethers::core::types::U256,
}
///`OrderInfo(address,address,uint256,uint256,address,bytes)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct OrderInfo {
    pub reactor: ::ethers::core::types::Address,
    pub swapper: ::ethers::core::types::Address,
    pub nonce: ::ethers::core::types::U256,
    pub deadline: ::ethers::core::types::U256,
    pub additional_validation_contract: ::ethers::core::types::Address,
    pub additional_validation_data: ::ethers::core::types::Bytes,
}
///`OutputToken(address,uint256,address)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct OutputToken {
    pub token: ::ethers::core::types::Address,
    pub amount: ::ethers::core::types::U256,
    pub recipient: ::ethers::core::types::Address,
}
///`ResolvedOrder((address,address,uint256,uint256,address,bytes),(address,uint256,uint256),(address,uint256,address)[],bytes,bytes32)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct ResolvedOrder {
    pub info: OrderInfo,
    pub input: InputToken,
    pub outputs: ::std::vec::Vec<OutputToken>,
    pub sig: ::ethers::core::types::Bytes,
    pub hash: [u8; 32],
}
///`FuzzSelector(address,bytes4[])`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct FuzzSelector {
    pub addr: ::ethers::core::types::Address,
    pub selectors: ::std::vec::Vec<[u8; 4]>,
}
