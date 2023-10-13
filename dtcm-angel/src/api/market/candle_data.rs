use chrono::NaiveDateTime;
use dtcm_angel_utils::date::{from_yyyy_mm_dd_hh_mm, serde_yyyy_mm_dd_hh_mm};

use crate::{
    types::{Interval, MarketDataExchange},
    Result,
};

/// Candle data request
#[derive(Debug, Serialize)]
#[api(POST, CandleData)]
pub struct CandleDataReq {
    /// Exchange to get the data from
    pub exchange: MarketDataExchange,
    /// Symbol token
    #[serde(rename = "symboltoken")]
    pub symbol_token: String,
    /// Interval for the data
    pub interval: Interval,
    /// From date
    #[serde(rename = "fromdate", serialize_with = "serde_yyyy_mm_dd_hh_mm")]
    pub from_date: NaiveDateTime,
    /// To date
    #[serde(rename = "todate", serialize_with = "serde_yyyy_mm_dd_hh_mm")]
    pub to_date: NaiveDateTime,
}

/// Candle data response
#[derive(Debug, Deserialize)]
pub struct CandleDataRes(Vec<(String, f64, f64, f64, f64, usize)>);

impl CandleDataReq {
    /// Returns a new instance for the candle data
    pub fn new<S, F, T>(
        exchange: MarketDataExchange,
        symbol_token: S,
        interval: Interval,
        from_date: F,
        to_date: T,
    ) -> Result<Self>
    where
        S: Into<String>,
        F: AsRef<str>,
        T: AsRef<str>,
    {
        Ok(Self {
            exchange,
            symbol_token: symbol_token.into(),
            interval,
            from_date: from_yyyy_mm_dd_hh_mm(from_date)?,
            to_date: from_yyyy_mm_dd_hh_mm(to_date)?,
        })
    }
}
