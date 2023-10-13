use crate::types::{DurationType, ExchangeType, ProductType, TransactionType};

/// Convert or change a position's margin product
#[allow(missing_docs)]
#[derive(Debug, Serialize)]
#[api(POST, ConvertPosition)]
pub struct ConvertPositionReq {
    #[serde(rename = "exchange")]
    pub exchange: ExchangeType,
    #[serde(rename = "oldproducttype")]
    pub old_product_type: ProductType,
    #[serde(rename = "newproducttype")]
    pub new_product_type: ProductType,
    #[serde(rename = "tradingsymbol")]
    pub trading_symbol: String,
    #[serde(rename = "transactiontype")]
    pub transaction_type: TransactionType,
    #[serde(rename = "quantity")]
    pub quantity: usize,
    #[serde(rename = "type")]
    pub duration: DurationType,
    // #[serde(rename = "symboltoken")]
    // pub symbol_token: String,
    // #[serde(rename = "symbolname")]
    // pub symbol_name: String,
    // #[serde(rename = "instrumenttype")]
    // pub instrument_type: String,
    // #[serde(rename = "priceden")]
    // pub price_den: String,
    // #[serde(rename = "pricenum")]
    // pub price_num: String,
    // #[serde(rename = "genden")]
    // pub gen_den: String,
    // #[serde(rename = "gennum")]
    // pub gen_num: String,
    // #[serde(rename = "precision")]
    // pub precision: String,
    // #[serde(rename = "multiplier")]
    // pub multiplier: String,
    // #[serde(rename = "boardlotsize")]
    // pub board_lot_size: String,
    // #[serde(rename = "buyqty")]
    // pub buy_qty: String,
    // #[serde(rename = "sellqty")]
    // pub sell_qty: String,
    // #[serde(rename = "buyamount")]
    // pub buy_amount: String,
    // #[serde(rename = "sellamount")]
    // pub sell_amount: String,
}

impl ConvertPositionReq {
    /// Returns a new instance for the convert position request
    pub fn new<T>(trading_symbol: T, quantity: usize) -> Self
    where
        T: Into<String>,
    {
        Self {
            exchange: Default::default(),
            old_product_type: Default::default(),
            new_product_type: Default::default(),
            trading_symbol: trading_symbol.into(),
            transaction_type: Default::default(),
            quantity,
            duration: Default::default(),
        }
    }

    /// Sets the exchange for the convert position request
    pub fn exchange(mut self, exchange: ExchangeType) -> Self {
        self.exchange = exchange;
        self
    }
    /// Sets the old_product_type for the convert position request
    pub fn old_product_type(mut self, old_product_type: ProductType) -> Self {
        self.old_product_type = old_product_type;
        self
    }
    /// Sets the new_product_type for the convert position request
    pub fn new_product_type(mut self, new_product_type: ProductType) -> Self {
        self.new_product_type = new_product_type;
        self
    }
    /// Sets the transaction_type for the convert position request
    pub fn transaction_type(mut self, transaction_type: TransactionType) -> Self {
        self.transaction_type = transaction_type;
        self
    }
    /// Sets the duration for the convert position request
    pub fn duration(mut self, duration: DurationType) -> Self {
        self.duration = duration;
        self
    }
}
