use response_types::{OrderResponse, OrderResponseInner, OrderStatus};
pub mod response_types;
use super::OrderType;
use crate::{
    contracts::internal::{
        dutch::DutchOrder, exclusive_dutch::ExclusiveDutchOrder, limit::LimitOrder,
    },
    order::{Order, OrderInner},
    subscriber::OrderSubscriber,
};
use alloy_sol_types::{Error as AlloySolTypeError, SolType};
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

fn clean_encoding(s: &str) -> &str {
    &s[66..]
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
