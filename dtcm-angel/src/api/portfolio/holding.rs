use crate::types::{ExchangeType, ProductType};

/// Placeholder containing holding information
#[allow(missing_docs)]
#[derive(Debug, Deserialize)]
#[api(GET, Holding)]
pub struct Holding {
    #[serde(rename = "tradingsymbol")]
    pub trading_symbol: String,
    pub exchange: ExchangeType,
    pub isin: String,
    pub t1quantity: usize,
    #[serde(rename = "realisedquantity")]
    pub realized_quantity: usize,
    pub quantity: usize,
    #[serde(rename = "authorisedquantity")]
    pub authorized_quantity: usize,
    pub product: ProductType,
    #[serde(rename = "collateralquantity")]
    pub collateral_quantity: Option<usize>,
    #[serde(rename = "collateraltype")]
    pub collateral_type: Option<usize>,
    pub haircut: f64,
    #[serde(rename = "averageprice")]
    pub average_price: f64,
    pub ltp: f64,
    #[serde(rename = "symboltoken")]
    pub symbol_token: String,
    pub close: f64,
    #[serde(rename = "profitandloss")]
    pub profit_and_loss: f64,
    #[serde(rename = "pnlpercentage")]
    pub pnl_percentage: f64,
}
