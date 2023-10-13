use dtcm_angel_utils::http::{EndPoint, HttpClient};

use std::fmt::Display;

use crate::Result;

use super::{OrderInner, OrderSetter};

/// Modify order request
#[derive(Debug, Serialize)]
pub struct ModifyOrderReq {
    /// Order ID
    #[serde(rename = "orderid")]
    pub order_id: String,
    /// Common inner structure for [`ModifyOrderReq`] and [`super::PlaceOrderReq`]
    #[serde(flatten)]
    pub inner: OrderInner,
}

/// Modify order response
#[derive(Debug, Deserialize)]
pub struct ModifyOrderRes {
    /// Order ID
    #[serde(rename = "orderid")]
    pub order_id: String,
}

impl ModifyOrderReq {
    /// Returns a new instance for [`ModifyOrderReq`]
    pub fn new<S, T, O>(trading_symbol: S, symbol_token: T, order_id: O) -> Self
    where
        S: Into<String>,
        T: Into<String>,
        O: Display,
    {
        Self {
            order_id: order_id.to_string(),
            inner: OrderInner::new(trading_symbol, symbol_token),
        }
    }

    /// Sends the [`ModifyOrderReq`] to the API and returns [`ModifyOrderRes`]
    pub async fn send(&self, http: &HttpClient) -> Result<ModifyOrderRes> {
        http.post(EndPoint::OrderModify, self)
            .await
            .and_then(|res| res.into_data())
    }
}

impl OrderSetter for ModifyOrderReq {
    fn inner_mut(&mut self) -> &mut OrderInner {
        &mut self.inner
    }
}
