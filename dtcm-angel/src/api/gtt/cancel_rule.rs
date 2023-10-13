use crate::types::ExchangeType;

/// Cancel rule request
#[derive(Debug, Serialize)]
#[api(POST, GttCancel)]
pub struct CancelRuleReq {
    /// Rule ID
    pub id: String,
    /// Symbol token
    #[serde(rename = "symboltoken")]
    pub symbol_token: String,
    /// Exchange
    pub exchange: ExchangeType,
}

/// Cancel rule response
#[derive(Debug, Deserialize)]
pub struct CancelRuleRes {
    /// Rule ID
    pub id: String,
}

impl CancelRuleReq {
    /// Returns a new instance for the [`CancelRuleReq`]
    pub fn new<I, S>(id: I, symbol_token: S, exchange: ExchangeType) -> Self
    where
        I: Into<String>,
        S: Into<String>,
    {
        Self {
            id: id.into(),
            symbol_token: symbol_token.into(),
            exchange,
        }
    }
}
