/// Rule detail request
#[derive(Debug, Serialize)]
#[api(POST, GttDetails)]
pub struct RuleDetailReq {
    /// Rule ID
    pub id: String,
}

/// Rule detail response
#[derive(Debug, Deserialize)]
pub struct RuleDetailRes {
    /// Rule status
    pub status: String,
    /// Created date
    #[serde(rename = "createddate")]
    pub created_date: String,
    /// Updated date
    #[serde(rename = "updateddate")]
    pub updated_date: String,
    /// Expiry date
    #[serde(rename = "expirydate")]
    pub expiry_date: String,
    /// Client ID
    #[serde(rename = "clientid")]
    pub client_id: String,
    /// Trading symbol
    #[serde(rename = "tradingsymbol")]
    pub trading_symbol: String,
    /// Symbol token
    #[serde(rename = "symboltoken")]
    pub symbol_token: String,
    /// Exchange
    #[serde(rename = "exchange")]
    pub exchange: String,
    /// Transaction type
    #[serde(rename = "transactiontype")]
    pub transaction_type: String,
    /// Product type
    #[serde(rename = "producttype")]
    pub product_type: String,
    /// Price
    pub price: String,
    /// Quantity
    pub qty: String,
    /// Trigger price
    #[serde(rename = "triggerprice")]
    pub trigger_price: String,
    /// Disclosed quantity
    #[serde(rename = "disclosedqty")]
    pub disclosed_qty: String,
}

impl RuleDetailReq {
    /// Return a new instance for the [`RuleDetailReq`]
    pub fn new<I>(id: I) -> Self
    where
        I: Into<String>,
    {
        Self { id: id.into() }
    }
}
