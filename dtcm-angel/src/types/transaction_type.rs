/// Transaction Types
#[derive(Debug, Serialize, Deserialize)]
pub enum TransactionType {
    #[serde(rename = "BUY")]
    /// Buy
    Buy,
    #[serde(rename = "SELL")]
    /// Sell
    Sell,
}

impl Default for TransactionType {
    fn default() -> Self {
        Self::Buy
    }
}
