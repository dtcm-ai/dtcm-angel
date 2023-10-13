/// Rule types
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum RuleType {
    /// New Rule
    #[serde(rename = "NEW")]
    New,
    /// Rule is cancelled
    #[serde(rename = "CANCELLED")]
    Cancelled,
    /// Rule is active
    #[serde(rename = "ACTIVE")]
    Active,
    /// Rule sent to exchange
    #[serde(rename = "SENTTOEXCHANGE")]
    SentToExchange,
    /// For all Rule
    #[serde(rename = "FORALL")]
    ForAll,
}
