/// Exchange Type
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ExchangeType {
    /// BSE Equity
    BSE,
    /// NSE Equity
    NSE,
    /// NSE Future and Options
    NFO,
    /// MCX Commodity
    MCX,
}

impl Default for ExchangeType {
    fn default() -> Self {
        Self::NSE
    }
}

/// Exchange type for market data requests
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MarketDataExchange {
    /// NSE Equity
    NSE,
    /// NSE Future and Options
    NFO,
}

impl Default for MarketDataExchange {
    fn default() -> Self {
        Self::NSE
    }
}
