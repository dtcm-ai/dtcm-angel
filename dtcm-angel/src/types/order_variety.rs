/// Order Variety
#[derive(Debug, Serialize, Deserialize)]
pub enum OrderVariety {
    /// Normal Order (Regular)
    #[serde(rename = "NORMAL")]
    Normal,
    /// Stop loss order
    #[serde(rename = "STOPLOSS")]
    StopLoss,
    /// After Market Order
    #[serde(rename = "AMO")]
    Amo,
    #[serde(rename = "ROBO")]
    /// ROBO (Bracket Order)
    Robo,
}

impl Default for OrderVariety {
    fn default() -> Self {
        Self::Normal
    }
}
