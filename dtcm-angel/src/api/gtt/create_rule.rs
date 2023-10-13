use std::fmt::Display;

use crate::types::{ExchangeType, ProductType, TransactionType};

/// [`CreateRuleReq`]
#[derive(Debug, Serialize)]
#[api(POST, GttCreate)]
pub struct CreateRuleReq {
    /// Rule ID
    pub id: String,
    #[serde(rename = "tradingsymbol")]
    /// Trading symbol
    pub trading_symbol: String,
    /// Symbol token
    #[serde(rename = "symboltoken")]
    pub symbol_token: String,
    /// Exchange
    pub exchange: ExchangeType,
    /// Transaction type
    #[serde(rename = "transactiontype")]
    pub transaction_type: TransactionType,
    /// Product type
    #[serde(rename = "producttype")]
    pub product_type: ProductType,
    /// Price
    pub price: String,
    /// Quantity
    pub qty: String,
    #[serde(rename = "triggerprice")]
    /// Trigger price
    pub trigger_price: String,
    /// Disclosed quantity
    #[serde(rename = "disclosedqty")]
    pub disclosed_qty: String,
    /// Time period
    #[serde(rename = "timeperiod")]
    pub time_period: String,
}

/// Create rule response
#[derive(Debug, Deserialize)]
pub struct CreateRuleRes {
    /// Rule ID
    pub id: String,
}

impl CreateRuleReq {
    /// Returns a new instance for the [`CreateRuleReq`]
    pub fn new<I, S, T>(id: I, trading_symbol: S, symbol_token: T) -> Self
    where
        I: Into<String>,
        S: Into<String>,
        T: Into<String>,
    {
        Self {
            id: id.into(),
            trading_symbol: trading_symbol.into(),
            symbol_token: symbol_token.into(),
            exchange: Default::default(),
            transaction_type: Default::default(),
            product_type: Default::default(),
            price: String::from("0"),
            qty: String::from("0"),
            trigger_price: String::from("0"),
            disclosed_qty: String::from("0"),
            time_period: String::from("0"),
        }
    }

    /// Sets the [`ExchangeType`] for the [`CreateRuleReq`]
    pub fn exchange(mut self, exchange: ExchangeType) -> Self {
        self.exchange = exchange;
        self
    }
    /// Sets the [`TransactionType`] for the [`CreateRuleReq`]
    pub fn transaction_type(mut self, transaction_type: TransactionType) -> Self {
        self.transaction_type = transaction_type;
        self
    }
    /// Sets the [`ProductType`] for the [`CreateRuleReq`]
    pub fn product_type(mut self, product_type: ProductType) -> Self {
        self.product_type = product_type;
        self
    }
    /// Sets the price value for the [`CreateRuleReq`]
    pub fn price<T>(mut self, price: T) -> Self
    where
        T: Display,
    {
        self.price = price.to_string();
        self
    }

    /// Sets the qty value for the [`CreateRuleReq`]
    pub fn qty<T>(mut self, qty: T) -> Self
    where
        T: Display,
    {
        self.qty = qty.to_string();
        self
    }

    /// Sets the trigger_price value for the [`CreateRuleReq`]
    pub fn trigger_price<T>(mut self, trigger_price: T) -> Self
    where
        T: Display,
    {
        self.trigger_price = trigger_price.to_string();
        self
    }

    /// Sets the disclosed_qty value for the [`CreateRuleReq`]
    pub fn disclosed_qty<T>(mut self, disclosed_qty: T) -> Self
    where
        T: Display,
    {
        self.disclosed_qty = disclosed_qty.to_string();
        self
    }

    /// Sets the time_period value for the [`CreateRuleReq`]
    pub fn time_period<T>(mut self, time_period: T) -> Self
    where
        T: Display,
    {
        self.time_period = time_period.to_string();
        self
    }
}
