use crate::types::{DurationType, ExchangeType, OrderType, OrderVariety, ProductType};

use super::OrderSetter;

#[allow(missing_docs)]
#[derive(Debug, Serialize)]
pub struct OrderInner {
    pub variety: OrderVariety,
    /// Trading Symbol of the instrument
    #[serde(rename = "tradingsymbol")]
    pub trading_symbol: String,
    /// Symbol Token is unique identifier
    #[serde(rename = "symboltoken")]
    pub symbol_token: String,
    /// Name of the exchange
    #[serde(rename = "exchange")]
    pub exchange: ExchangeType,
    /// Order type (MARKET, LIMIT etc.)
    #[serde(rename = "ordertype")]
    pub order_type: OrderType,
    /// Product type (CNC,MIS)
    #[serde(rename = "producttype")]
    pub product_type: ProductType,
    /// Order duration (DAY,IOC)
    #[serde(rename = "duration")]
    pub duration: DurationType,
    /// The min or max price to execute the order at (for LIMIT orders)
    #[serde(rename = "price")]
    pub price: String,
    /// The price at which an order should be triggered (SL, SL-M)
    #[serde(rename = "triggerprice", skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<String>,
    /// Only For ROBO (Bracket Order)
    #[serde(rename = "squareoff", skip_serializing_if = "Option::is_none")]
    pub square_off: Option<String>,
    /// Only For ROBO (Bracket Order)
    #[serde(rename = "stoploss", skip_serializing_if = "Option::is_none")]
    pub stop_loss: Option<String>,
    /// Only For ROBO (Bracket Order)
    #[serde(rename = "trailingStopLoss", skip_serializing_if = "Option::is_none")]
    pub trailing_stop_loss: Option<String>,
    /// Quantity to disclose publicly (for equity trades)
    #[serde(rename = "disclosedquantity")]
    pub disclosed_quantity: String,
    /// Quantity to transact
    #[serde(rename = "quantity")]
    pub quantity: String,
}

impl OrderInner {
    /// Returns a new instance for [`OrderInner`]
    pub fn new<S, T>(trading_symbol: S, symbol_token: T) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        let trading_symbol = trading_symbol.into();
        let symbol_token = symbol_token.into();
        Self {
            variety: Default::default(),
            trading_symbol,
            symbol_token,
            exchange: Default::default(),
            order_type: Default::default(),
            product_type: Default::default(),
            duration: Default::default(),
            price: String::from("0"),
            trigger_price: None,
            square_off: None,
            stop_loss: None,
            trailing_stop_loss: None,
            disclosed_quantity: String::from("0"),
            quantity: String::from("0"),
        }
    }
}

impl OrderSetter for OrderInner {
    fn inner_mut(&mut self) -> &mut OrderInner {
        self
    }
}
