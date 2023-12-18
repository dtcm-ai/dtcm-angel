use super::Holding;

/// Comprehensive view of your entire investments, including individual stock holdings
/// and a summary of your total investments. In addition to the updates for individual stock holdings,
#[derive(Debug, Deserialize)]
#[api(GET, AllHolding)]
pub struct AllHoldings {
    /// Holdings
    pub holdings: Vec<Holding>,
    /// summary of your entire investments
    #[serde(rename = "totalholding")]
    pub total_holding: TotalHolding,
}

/// Provides a summary of your entire investments
#[derive(Debug, Deserialize)]
pub struct TotalHolding {
    ///  The total value of all your holdings.
    #[serde(rename = "totalholdingvalue")]
    pub total_holding_value: f64,
    /// The total investment value.
    #[serde(rename = "totalinvvalue")]
    pub total_inv_value: f64,
    /// The total profit and loss across all holdings.
    #[serde(rename = "totalprofitandloss")]
    pub total_profit_and_loss: f64,
    /// The total profit and loss percentage for your entire portfolio.
    #[serde(rename = "totalpnlpercentage")]
    pub total_pnl_percentage: f64,
}
