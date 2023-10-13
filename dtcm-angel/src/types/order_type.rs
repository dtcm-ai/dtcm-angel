/// Order type
#[derive(Debug, Serialize, Deserialize)]
pub enum OrderType {
    #[serde(rename = "MARKET")]
    /// Market Order(MKT)
    Market,
    #[serde(rename = "LIMIT")]
    /// Limit Order(L)
    Limit,
    #[serde(rename = "STOPLOSS_LIMIT")]
    /// Stop Loss Limit Order(SL)
    StopLossLimit,
    #[serde(rename = "STOPLOSS_MARKET")]
    /// Stop Loss Market Order(SL-M)
    StopLossMarket,
}

impl Default for OrderType {
    fn default() -> Self {
        Self::Market
    }
}
