use crate::types::ExchangeType;

/// Seac
#[derive(Debug, Serialize)]
#[api(POST, SearchScrip)]
pub struct SearchScripReq {
    exchange: ExchangeType,
    #[serde(rename = "searchscrip")]
    scrip: String,
}

/// Searched scrip
#[derive(Debug, Deserialize)]
pub struct SearchScrip {
    /// Searched symbol
    #[serde(rename = "tradingsymbol")]
    pub symbol: String,
    /// Exchange name
    pub exchange: ExchangeType,
    /// Searched token
    #[serde(rename = "symboltoken")]
    pub token: String,
}

/// Searched scrip
#[derive(Debug, Deserialize)]
pub struct SearchScripRes(Vec<SearchScrip>);

impl SearchScripReq {
    /// Returns a new instance for the search scrip request
    pub fn new<S>(exchange: ExchangeType, scrip: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            exchange,
            scrip: scrip.into(),
        }
    }
}
