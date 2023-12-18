use std::collections::HashMap;

use crate::types::{ExchangeType, MarketMode};

/// Market data request
#[derive(Debug, Serialize)]
#[api(POST, MarketData)]
pub struct MarketDataReq {
    /// Market data mode
    pub mode: MarketMode,
    /// Exchange tokens
    #[serde(rename = "exchangeTokens")]
    pub exchange_tokens: HashMap<ExchangeType, Vec<String>>,
}

/// Provides real-time market data for specific symbols
#[derive(Debug, Deserialize)]
pub struct MarketDataResInner {
    /// The exchange for the fetched data.
    pub exchange: String,
    /// The trading symbol for the fetched data.
    #[serde(rename = "tradingSymbol")]
    pub trading_symbol: String,
    /// The token for the fetched symbol.
    #[serde(rename = "symbolToken")]
    pub symbol_token: String,
    /// The last trading price for the fetched symbol.
    pub ltp: f64,
    #[serde(flatten)]
    pub ohlc: Option<Ohlc>,
    #[serde(flatten)]
    pub data: Option<FullData>,
}

#[derive(Debug, Deserialize)]
pub struct Ohlc {
    /// The opening price for the fetched symbol.
    pub open: f64,
    /// The highest price for the fetched symbol.
    pub high: f64,
    /// The lowest price for the fetched symbol.
    pub low: f64,
    /// The previous closing price for the fetched symbol.
    pub close: f64,
}

#[derive(Debug, Deserialize)]
pub struct OrderDepthInner {
    pub price: f64,
    pub quantity: usize,
    pub orders: usize,
}

#[derive(Debug, Deserialize)]
pub struct OrderDepth {
    /// Array of buy depth objects.
    pub buy: Vec<OrderDepthInner>,
    /// Array of sell depth objects.
    pub sell: Vec<OrderDepthInner>,
}

#[derive(Debug, Deserialize)]
pub struct FullData {
    ///	The quantity of the last trade executed for the fetched symbol.
    #[serde(rename = "lastTradeQty")]
    pub last_trade_qty: i64,
    ///	The exchange feed time for the fetched symbol.
    #[serde(rename = "exchFeedTime")]
    pub exch_feed_time: String,
    ///	The exchange trade time for the fetched symbol.
    #[serde(rename = "exchTradeTime")]
    pub exch_trade_time: String,
    ///	The net change for the fetched symbol.
    #[serde(rename = "netChange")]
    pub net_change: f64,
    ///	The percent change for the fetched symbol.
    #[serde(rename = "percentChange")]
    pub percent_change: f64,
    ///	The average price for the fetched symbol.
    #[serde(rename = "avgPrice")]
    pub avg_price: f64,
    ///	The trade volume for the fetched symbol.
    #[serde(rename = "tradeVolume")]
    pub trade_volume: i64,
    ///	The open interest for the fetched symbol.
    #[serde(rename = "opnInterest")]
    pub opn_interest: i64,
    ///	Maximum price increase allowed before trading pauses temporarily.
    #[serde(rename = "upperCircuit")]
    pub upper_circuit: f64,
    ///	Maximum price decrease allowed before trading pauses temporarily.
    #[serde(rename = "lowerCircuit")]
    pub lower_circuit: f64,
    ///	The total buy quantity for the fetched symbol.
    #[serde(rename = "totBuyQuan")]
    pub tot_buy_quan: i64,
    ///	The total sell quantity for the fetched symbol.
    #[serde(rename = "totSellQuan")]
    pub tot_sell_quan: i64,
    ///	The yearly highest price for the fetched symbol.
    #[serde(rename = "52WeekHigh")]
    pub week_52_high: f64,
    ///	The yearly lowest price for the fetched symbol.
    #[serde(rename = "52WeekLow")]
    pub week_52_low: f64,
    pub depth: OrderDepth,
}
#[derive(Debug, Deserialize)]
pub struct UnFetchedRes {
    pub exchange: String,
    #[serde(rename = "symbolToken")]
    pub symbol_token: String,
    pub message: String,
    #[serde(rename = "errorCode")]
    pub error_code: String,
}

/// Market data response
#[derive(Debug, Deserialize)]
pub struct MarketDataRes {
    /// fetched market data
    pub fetched: Vec<MarketDataResInner>,
    /// Unfetched data with errors
    pub unfetched: Vec<UnFetchedRes>,
}

impl MarketDataReq {
    /// Returns a new instance for the market data
    pub fn new(mode: MarketMode, exchange_tokens: HashMap<ExchangeType, Vec<String>>) -> Self {
        Self {
            mode,
            exchange_tokens,
        }
    }
}
