/// Market mode
#[derive(Debug, Serialize, Deserialize)]
pub enum MarketMode {
    /// Full data
    #[serde(rename = "FULL")]
    Full,
    /// OHLC data
    #[serde(rename = "OHLC")]
    Ohlc,
    /// Last traded price
    #[serde(rename = "LTP")]
    Ltp,
}

impl Default for MarketMode {
    fn default() -> Self {
        Self::Ltp
    }
}
