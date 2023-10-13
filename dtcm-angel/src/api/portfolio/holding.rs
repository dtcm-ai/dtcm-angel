/// Placeholder containing holding information
#[allow(missing_docs)]
#[derive(Debug, Deserialize)]
#[api(GET, Holding)]
pub struct Holding {
    #[serde(rename = "tradingSymbol")]
    pub trading_symbol: String,
    pub exchange: String,
    pub isin: String,
    pub t1quantity: String,
    #[serde(rename = "realisedquantity")]
    pub realized_quantity: String,
    pub quantity: String,
    #[serde(rename = "authorisedquantity")]
    pub authorized_quantity: String,
    #[serde(rename = "profitandloss")]
    pub profit_and_loss: String,
    pub product: String,
    #[serde(rename = "collateralquantity")]
    pub collateral_quantity: String,
    #[serde(rename = "collateraltype")]
    pub collateral_type: String,
    pub haircut: String,
}
