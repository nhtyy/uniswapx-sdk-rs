use ethers::{
    abi::{self, AbiDecode},
    types::Bytes,
};
use response_types::{OrderResponse, OrderResponseInner, OrderStatus};
pub mod response_types;
use super::OrderType;
use crate::{
    contracts::internal::{DutchOrder, ExclusiveDutchOrder, LimitOrder},
    order::{Order, OrderInner},
    subscriber::OrderSubscriber,
};
use futures::{stream, Stream};
use reqwest::{Client, Url};

const URL: &str = "https://api.uniswap.org/v2/orders";

pub struct OrderClient {
    // cache: Vec<Order>,
    // idx: usize,
    client: Client,
    url: Url,
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

impl OrderClient {
    pub fn new() -> Self {
        Self {
            // cache: Vec::new(),
            // idx: 0,
            client: Client::new(),
            url: Url::parse(URL).expect("URL to parse"),
        }
    }

    pub async fn get_orders(&self, params: ApiParams) -> Result<OrderResponse, reqwest::Error> {
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

impl OrderSubscriber for OrderClient {
    type InternalErr = reqwest::Error;

    fn subscribe(self) -> Box<dyn Stream<Item = Result<Order, Self::InternalErr>>> {
        Box::new(stream::unfold(self, |client| async move { todo!() }))
    }
}

impl TryFrom<OrderResponseInner> for DutchOrder {
    type Error = abi::AbiError;

    fn try_from(order: OrderResponseInner) -> Result<Self, Self::Error> {
        DutchOrder::decode_hex(order.encoded_order)
    }
}

impl TryFrom<OrderResponseInner> for LimitOrder {
    type Error = abi::AbiError;

    fn try_from(order: OrderResponseInner) -> Result<Self, Self::Error> {
        LimitOrder::decode_hex(order.encoded_order)
    }
}

impl TryFrom<OrderResponseInner> for ExclusiveDutchOrder {
    type Error = abi::AbiError;

    fn try_from(order: OrderResponseInner) -> Result<Self, Self::Error> {
        ExclusiveDutchOrder::decode_hex(order.encoded_order)
    }
}

impl TryFrom<OrderResponseInner> for Order {
    type Error = abi::AbiError;

    fn try_from(order: OrderResponseInner) -> Result<Self, Self::Error> {
        // todo!() probably need to do hex string to bytes
        // let sig = order.signature.clone();

        let sig: Bytes = Default::default();

        Ok(Self::new(
            match order.order_type {
                OrderType::Dutch => OrderInner::from(DutchOrder::try_from(order)?),
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
    type Error = abi::AbiError;

    fn try_from(response: OrderResponse) -> Result<Self, Self::Error> {
        response
            .orders
            .into_iter()
            .map(|order| Order::try_from(order))
            .collect()
    }
}
