use crate::types::ExchangeType;

/// LTP data request
#[derive(Debug, Serialize)]
#[api(POST, LtpData)]
pub struct LtpDataReq {
    /// Exchange
    #[serde(rename = "exchange")]
    pub exchange: ExchangeType,
    /// Trading symbol
    #[serde(rename = "tradingsymbol")]
    pub trading_symbol: String,
    /// Symbol token
    #[serde(rename = "symboltoken")]
    pub symbol_token: String,
}

/// /// LTP data response
#[derive(Debug, Deserialize)]
pub struct LtpDataRes {
    /// Exchange
    pub exchange: ExchangeType,
    /// Trading symbol
    #[serde(rename = "tradingsymbol")]
    pub trading_symbol: String,
    /// Symbol token
    #[serde(rename = "symboltoken")]
    pub symbol_token: String,
    /// Open price
    pub open: f64,
    /// High price
    pub high: f64,
    /// Low price
    pub low: f64,
    /// Close price
    pub close: f64,
    /// LTP price
    pub ltp: f64,
}

impl LtpDataReq {
    /// Returns a new instance for LTP Data
    pub fn new<S, T>(exchange: ExchangeType, trading_symbol: S, symbol_token: T) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        Self {
            exchange,
            trading_symbol: trading_symbol.into(),
            symbol_token: symbol_token.into(),
        }
    }
}
