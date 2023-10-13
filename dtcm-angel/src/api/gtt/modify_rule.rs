use std::fmt::Display;

use crate::types::ExchangeType;

/// Modify rule request
#[derive(Debug, Serialize)]
#[api(POST, GttModify)]
pub struct ModifyRuleReq {
    /// Rule ID
    pub id: String,
    /// Symbol token
    #[serde(rename = "symboltoken")]
    pub symbol_token: String,
    /// Exchange
    pub exchange: ExchangeType,
    /// Price
    pub price: String,
    /// Quantity
    pub qty: String,
    /// Trigger price
    #[serde(rename = "triggerprice")]
    pub trigger_price: String,
    /// Disclosed quantity
    #[serde(rename = "disclosedqty")]
    pub disclosed_qty: String,
    /// Time period
    #[serde(rename = "timeperiod")]
    pub time_period: String,
}

/// Modify rule response
#[derive(Debug, Deserialize)]
pub struct ModifyRuleRes {
    /// Order ID
    #[serde(rename = "orderid")]
    pub order_id: String,
}

impl ModifyRuleReq {
    /// Return a new instance for the [`ModifyRuleReq`]
    pub fn new<I, T>(id: I, symbol_token: T) -> Self
    where
        I: Into<String>,
        T: Into<String>,
    {
        Self {
            id: id.into(),
            symbol_token: symbol_token.into(),
            exchange: Default::default(),
            price: String::from("0"),
            qty: String::from("0"),
            trigger_price: String::from("0"),
            disclosed_qty: String::from("0"),
            time_period: String::from("0"),
        }
    }

    /// Sets the [`ExchangeType`] value for the [`ModifyRuleReq`]
    pub fn exchange(mut self, exchange: ExchangeType) -> Self {
        self.exchange = exchange;
        self
    }

    /// Sets the price value for the [`ModifyRuleReq`]
    pub fn price<T>(mut self, price: T) -> Self
    where
        T: Display,
    {
        self.price = price.to_string();
        self
    }

    /// Sets the qty value for the [`ModifyRuleReq`]
    pub fn qty<T>(mut self, qty: T) -> Self
    where
        T: Display,
    {
        self.qty = qty.to_string();
        self
    }

    /// Sets the trigger_price value for the [`ModifyRuleReq`]
    pub fn trigger_price<T>(mut self, trigger_price: T) -> Self
    where
        T: Display,
    {
        self.trigger_price = trigger_price.to_string();
        self
    }

    /// Sets the disclosed_qty value for the [`ModifyRuleReq`]
    pub fn disclosed_qty<T>(mut self, disclosed_qty: T) -> Self
    where
        T: Display,
    {
        self.disclosed_qty = disclosed_qty.to_string();
        self
    }

    /// Sets the time_period value for the [`ModifyRuleReq`]
    pub fn time_period<T>(mut self, time_period: T) -> Self
    where
        T: Display,
    {
        self.time_period = time_period.to_string();
        self
    }
}
