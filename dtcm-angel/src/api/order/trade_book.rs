use crate::types::{ExchangeType, ProductType, TransactionType};

/// Placeholder for the trade book
#[allow(missing_docs)]
#[derive(Debug, Deserialize)]
#[api(GET, TradeBook)]
pub struct TradeBook {
    pub exchange: ExchangeType,
    #[serde(rename = "producttype")]
    pub product_type: ProductType,
    #[serde(rename = "tradingsymbol")]
    pub trading_symbol: String,
    #[serde(rename = "instrumenttype")]
    pub instrument_type: String,
    #[serde(rename = "symbolgroup")]
    pub symbol_group: String,
    #[serde(rename = "strikeprice")]
    pub strike_price: String,
    #[serde(rename = "optiontype")]
    pub option_type: String,
    #[serde(rename = "expirydate")]
    pub expiry_date: String,
    #[serde(rename = "marketlot")]
    pub market_lot: String,
    pub precision: String,
    pub multiplier: String,
    #[serde(rename = "trade_value")]
    pub trade_value: String,
    #[serde(rename = "transactiontype")]
    pub transaction_type: TransactionType,
    #[serde(rename = "fillprice")]
    pub fill_price: String,
    #[serde(rename = "fillsize")]
    pub fill_size: String,
    #[serde(rename = "orderid")]
    pub order_id: String,
    #[serde(rename = "fillid")]
    pub fill_id: String,
    #[serde(rename = "filltime")]
    pub fill_time: String,
}
