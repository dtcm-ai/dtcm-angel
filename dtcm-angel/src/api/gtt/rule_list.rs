use crate::types::{ExchangeType, ProductType, RuleType, TransactionType};

/// Rule list request
#[derive(Debug, Serialize)]
#[api(POST, GttList)]
pub struct RuleListReq {
    /// Status
    pub status: Vec<RuleType>,
    /// Page
    pub page: usize,
    /// Count
    pub count: usize,
}

/// Rule list response
#[derive(Debug, Deserialize)]
pub struct RuleListRes {
    /// Client ID
    #[serde(rename = "clientid")]
    pub client_id: String,
    /// Created date
    #[serde(rename = "createddate")]
    pub created_date: String,
    /// Exchange
    #[serde(rename = "exchange")]
    pub exchange: ExchangeType,
    /// Product type
    #[serde(rename = "producttype")]
    pub product_type: ProductType,
    /// Transaction type
    #[serde(rename = "transactiontype")]
    pub transaction_type: TransactionType,
    /// Expiry date
    #[serde(rename = "expirydate")]
    pub expiry_date: String,
    /// ID
    pub id: String,
    /// Quantity
    pub qty: String,
    /// Price
    pub price: String,
    /// Status
    pub status: String,
    /// Symbol token
    #[serde(rename = "symboltoken")]
    pub symbol_token: String,
    /// Trading symbol
    #[serde(rename = "tradingsymbol")]
    pub trading_symbol: String,
    ///Trigger price
    #[serde(rename = "triggerprice")]
    pub trigger_price: String,
    /// Updated date
    #[serde(rename = "updateddate")]
    pub updated_date: String,
}

impl RuleListReq {
    /// Return a new instance for the [`RuleListReq`]
    pub fn new(status: Vec<RuleType>, page: usize, count: usize) -> Self {
        Self {
            status,
            page,
            count,
        }
    }
}
