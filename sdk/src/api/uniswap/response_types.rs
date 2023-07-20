use super::super::OrderType;
use serde::{Deserialize, Serialize};

/// idk where tf this comes from this is just the response from the api in the docs
#[derive(Deserialize, Debug)]
pub struct OrderResponseInner {
    #[serde(rename = "type")]
    pub order_type: OrderType,
    #[serde(rename = "orderStatus")]
    pub order_status: OrderStatus,
    #[serde(rename = "chainId")]
    pub chain_id: usize,
    #[serde(rename = "encodedOrder")]
    pub encoded_order: String,

    pub signature: String,
    #[serde(rename = "orderHash")]
    pub order_hash: String,
    #[serde(rename = "createdAt")]
    pub created_at: u128,

    pub input: OrderInput,
    pub outputs: Vec<OrderOutput>,
}

//https://github.com/Uniswap/uniswapx-service/blob/main/lib/entities/Order.ts
#[derive(Deserialize, Debug)]
pub struct OrderInput {
    pub token: String,
    #[serde(rename = "startAmount")]
    pub start_amount: Option<String>,
    #[serde(rename = "endAmount")]
    pub end_amount: Option<String>,
}

//https://github.com/Uniswap/uniswapx-service/blob/main/lib/entities/Order.ts
#[derive(Deserialize, Debug)]
pub struct OrderOutput {
    pub token: String,
    #[serde(rename = "startAmount")]
    pub start_amount: Option<String>,
    #[serde(rename = "endAmount")]
    pub end_amount: Option<String>,
    pub recipient: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct OrderResponse {
    pub orders: Vec<OrderResponseInner>,
    pub cursor: Option<String>,
}

//https://github.com/Uniswap/uniswapx-service/blob/main/lib/entities/Order.ts
#[derive(Serialize, Deserialize, Debug)]
pub enum OrderStatus {
    #[serde(rename = "open")]
    Open,
    Expired,
    Error,
    Cancelled,
    #[serde(rename = "filled")]
    Filled,
    InsufficientFunds,
}
