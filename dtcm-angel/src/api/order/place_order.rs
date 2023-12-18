use crate::types::TransactionType;

use super::{OrderInner, OrderSetter};

/// Place order request
#[derive(Debug, Serialize)]
#[api(POST, OrderPlace)]
pub struct PlaceOrderReq {
    /// BUY or SELL
    #[serde(rename = "transactiontype")]
    pub transaction_type: TransactionType,
    /// Inner order parameters
    #[serde(flatten)]
    pub inner: OrderInner,
}

/// Place order response
#[derive(Debug, Deserialize)]
pub struct PlaceOrderRes {
    /// Script name
    pub script: String,
    /// Order ID
    #[serde(rename = "orderid")]
    pub order_id: Option<String>,
    /// Unique order ID
    #[serde(rename = "uniqueorderid")]
    pub unique_order_id: Option<String>,
}

impl PlaceOrderReq {
    /// Returns a new instance for [`PlaceOrderReq`]
    pub fn new<S, T>(trading_symbol: S, symbol_token: T, transaction_type: TransactionType) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        Self {
            transaction_type,
            inner: OrderInner::new(trading_symbol, symbol_token),
        }
    }

    /// Sets the [`TransactionType`] for the [`PlaceOrderReq`]
    pub fn transaction_type(mut self, transaction_type: TransactionType) -> Self {
        self.transaction_type = transaction_type;
        self
    }
}

impl OrderSetter for PlaceOrderReq {
    fn inner_mut(&mut self) -> &mut OrderInner {
        &mut self.inner
    }
}
