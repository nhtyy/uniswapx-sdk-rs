/// the default response types from the uniswap api
pub mod response_types;

use super::client::Client;
use alloy_sol_types::Error as AlloySolTypeError;
use reqwest::{Client as ReqwestClient, Url};
use response_types::{OrderResponse, OrderResponseInner, OrderStatus};
use uniswapx_sdk_core::order::{OrderType, SignedOrder};
use uniswapx_sdk_core::{
    contracts::internal::{
        dutch::DutchOrder, exclusive_dutch::ExclusiveDutchOrder, limit::LimitOrder,
    },
    order::Order,
};

#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn};

const URL: &str = "https://api.uniswap.org/v2/orders";

pub struct UniswapClient {
    client: ReqwestClient,
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
            client: ReqwestClient::new(),
            url: Url::parse(URL).expect("URL to parse"),
            chain_id,
        }
    }

    /// todo! there are more params on thier server
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

#[async_trait::async_trait]
impl Client<SignedOrder> for UniswapClient {
    type ClientError = ClientError;

    /// returns as many open orders as possible
    async fn firehose(&self) -> Result<Vec<SignedOrder>, Self::ClientError> {
        let res = self
            .get_orders_with_params(ApiParams {
                limit: 10,
                chain_id: self.chain_id,
                order_status: OrderStatus::Open,
            })
            .await?;

        Ok(Vec::<SignedOrder>::try_from(res)?)
    }
}

// todo! uniswap api labels exlusive dutch as a dutch
impl TryFrom<OrderResponseInner> for SignedOrder {
    type Error = AlloySolTypeError;

    fn try_from(order: OrderResponseInner) -> Result<Self, Self::Error> {
        // clone because were gonna consumer order
        let sig = order.signature.clone();

        let order = match order.order_type {
            OrderType::Dutch => {
                warn!("uniswap api returned a dutch order, but it should be really an exclusive dutch order");
                Order::from(ExclusiveDutchOrder::try_from(order.encoded_order)?)
                // see todo
            }
            OrderType::Limit => Order::from(LimitOrder::try_from(order.encoded_order)?),
            OrderType::ExclusiveDutch => {
                Order::from(ExclusiveDutchOrder::try_from(order.encoded_order)?)
            }
        };

        Ok(order.signed(sig))
    }
}

impl TryFrom<OrderResponse> for Vec<SignedOrder> {
    type Error = AlloySolTypeError;

    fn try_from(response: OrderResponse) -> Result<Self, Self::Error> {
        response
            .orders
            .into_iter()
            .map(|order| SignedOrder::try_from(order))
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
