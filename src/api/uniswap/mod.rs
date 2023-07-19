use std::pin::Pin;
use response_types::{OrderResponse, OrderResponseInner, OrderStatus};
pub mod response_types;
use super::OrderClient;
use super::OrderType;
use crate::{
    contracts::internal::{
        dutch::DutchOrder, exclusive_dutch::ExclusiveDutchOrder, limit::LimitOrder,
    },
    order::{Order, OrderInner},
};
use alloy_sol_types::{Error as AlloySolTypeError, SolType};
use reqwest::{Client, Url};

const URL: &str = "https://api.uniswap.org/v2/orders";

pub struct UniswapClient {
    client: Client,
    url: Url,
    chain_id: usize,
}

#[derive(Debug)]
pub enum ClientError {
    Network(reqwest::Error),
    Encoding(AlloySolTypeError),
}

pub struct ApiParams {
    pub limit: usize,
    pub chain_id: usize,
    pub order_status: OrderStatus,
}

impl ApiParams {
    fn as_query_string(&self) -> String {
        todo!()
    }
}

impl UniswapClient {
    pub fn new(chain_id: usize) -> Self {
        Self {
            client: Client::new(),
            url: Url::parse(URL).expect("URL to parse"),
            chain_id,
        }
    }

    pub async fn get_orders_with_params(
        &self,
        params: ApiParams,
    ) -> Result<OrderResponse, reqwest::Error> {
        Ok(self
            .client
            .get(self.url.clone())
            .query(&[("limit", params.limit), ("chainId", params.chain_id)])
            .query(&[("orderStatus", params.order_status)]) // types
            .send()
            .await?
            .json::<OrderResponse>()
            .await?)
    }
}

impl OrderClient for UniswapClient {
    type ClientError = ClientError;

    /// hardcode 10 orders per batch
    fn get_open_orders(
        self,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<Vec<Order>, Self::ClientError>>>> {
        Box::pin(async move {
            let res = self
                .get_orders_with_params(ApiParams {
                    limit: 10,
                    chain_id: self.chain_id,
                    order_status: OrderStatus::Open,
                })
                .await?;

            Ok(Vec::try_from(res)?)
        })
    }
}

impl TryFrom<OrderResponseInner> for DutchOrder {
    type Error = AlloySolTypeError;

    fn try_from(order: OrderResponseInner) -> Result<Self, Self::Error> {
        DutchOrder::hex_decode(clean_encoding(&order.encoded_order), true)
    }
}

impl TryFrom<OrderResponseInner> for LimitOrder {
    type Error = AlloySolTypeError;

    fn try_from(order: OrderResponseInner) -> Result<Self, Self::Error> {
        LimitOrder::hex_decode(clean_encoding(&order.encoded_order), true)
    }
}

impl TryFrom<OrderResponseInner> for ExclusiveDutchOrder {
    type Error = AlloySolTypeError;

    fn try_from(order: OrderResponseInner) -> Result<Self, Self::Error> {
        ExclusiveDutchOrder::hex_decode(clean_encoding(&order.encoded_order), true)
    }
}

impl TryFrom<OrderResponseInner> for Order {
    type Error = AlloySolTypeError;

    // todo! uniswap api labels exlusive dutch as a dutch
    fn try_from(order: OrderResponseInner) -> Result<Self, Self::Error> {
        let sig = order.signature.clone();

        Ok(Self::new(
            match order.order_type {
                OrderType::Dutch => OrderInner::from(ExclusiveDutchOrder::try_from(order)?),
                OrderType::Limit => OrderInner::from(LimitOrder::try_from(order)?),
                OrderType::ExclusiveDutch => {
                    OrderInner::from(ExclusiveDutchOrder::try_from(order)?)
                }
            },
            sig,
        ))
    }
}

/// compiler magic
impl TryFrom<OrderResponse> for Vec<Order> {
    type Error = AlloySolTypeError;

    fn try_from(response: OrderResponse) -> Result<Self, Self::Error> {
        response
            .orders
            .into_iter()
            .map(|order| Order::try_from(order))
            .collect()
    }
}

impl std::error::Error for ClientError {}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientError::Network(e) => write!(f, "Network error: {}", e),
            ClientError::Encoding(e) => write!(f, "Encoding error: {}", e),
        }
    }
}

impl From<reqwest::Error> for ClientError {
    fn from(e: reqwest::Error) -> Self {
        Self::Network(e)
    }
}

impl From<AlloySolTypeError> for ClientError {
    fn from(e: AlloySolTypeError) -> Self {
        Self::Encoding(e)
    }
}

fn clean_encoding(s: &str) -> &str {
    &s[66..]
}
