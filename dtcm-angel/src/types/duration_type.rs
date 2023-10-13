/// Duration type
#[derive(Debug, Serialize, Deserialize)]
pub enum DurationType {
    #[serde(rename = "DAY")]
    /// Regular Order
    Day,
    #[serde(rename = "IOC")]
    /// Immediate or Cancel
    Ioc,
}

impl Default for DurationType {
    fn default() -> Self {
        Self::Day
    }
}
