use crate::types::{ExchangeType, ProductType, TransactionType};

/// Margin calculation request
#[api(POST, MarginApi)]
#[derive(Debug, Serialize)]
pub struct MarginCalculatorReq {
    /// Total positions for margin calculation
    pub positions: Vec<MarginCalculatorPosition>,
}

/// Position for margin calculation
#[derive(Debug, Serialize, Clone)]
pub struct MarginCalculatorPosition {
    /// This is an enumeration field with values like NSE, BSE, NFO, CDS, MCX, NCDEX and BFO.
    /// It's used to specify the exchange where the instrument is traded.
    pub exchange: ExchangeType,
    /// Enter the quantity of the instrument you want to buy or sell.
    /// In the NFO segment, specify the number of units within a lot,
    /// not the number of lots. For instance, if you plan to trade one lot of
    /// Nifty Futures (comprising 50 units), set the quantity as 50. This maintains
    /// consistency with place order requests for NFO instruments and ensures precise
    /// margin calculations.
    #[serde(rename = "qty")]
    pub quantity: usize,
    /// If applicable, use this field to specify the desired price for your transaction.
    /// If a specific price isn't relevant, you can set it to 0.
    pub price: f64,
    /// This field enables you to define the product type for your transaction.
    /// It's another enumeration field with options such as CARRYFORWARD, INTRADAY, DELIVERY, MARGIN, BO, and CO.
    #[serde(rename = "productType")]
    pub product_type: ProductType,
    /// Use this field to provide the symbol token.
    /// This token uniquely identifies the symbol you're trading.
    pub token: String,
    /// This is also an enumeration field with values BUY or SELL.
    /// It's used to indicate whether you're buying or selling the instrument.
    #[serde(rename = "tradeType")]
    pub trade_type: TransactionType,
}

/// Margin calculation response
#[derive(Debug, Deserialize, Clone)]
pub struct MarginCalculatorRes {
    /// Total margin required
    #[serde(rename = "totalMarginRequired")]
    pub total_margin_required: f64,
    /// Margin components
    #[serde(rename = "marginComponents")]
    pub margin_components: MarginComponents,
    /// Margin breakups
    #[serde(rename = "marginBreakup")]
    pub margin_breakup: Vec<MarginBreakup>,
    /// Options buy
    #[serde(rename = "optionsBuy")]
    pub options_buy: OptionsBuy,
}

/// MarginComponents
#[derive(Debug, Deserialize, Clone)]
pub struct MarginComponents {
    #[serde(rename = "netPremium")]
    pub net_premium: f64,
    #[serde(rename = "spanMargin")]
    pub span_margin: f64,
    #[serde(rename = "marginBenefit")]
    pub margin_benefit: f64,
    #[serde(rename = "deliveryMargin")]
    pub delivery_margin: f64,
    #[serde(rename = "nonNFOMargin")]
    pub non_nfo_margin: f64,
    #[serde(rename = "totOptionsPremium")]
    pub total_options_premium: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MarginBreakup {
    pub exchange: ExchangeType,
    #[serde(rename = "productType")]
    pub product_type: ProductType,
    #[serde(rename = "totalMarginRequired")]
    pub total_margin_required: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OptionsBuy {
    #[serde(rename = "totOptionsPremium")]
    pub total_options_premium: f64,
    #[serde(rename = "optionDetails")]
    pub option_details: Vec<OptionDetail>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OptionDetail {
    pub exchange: ExchangeType,
    #[serde(rename = "productType")]
    pub product_type: ProductType,
    pub token: String,
    #[serde(rename = "lotMultiplier")]
    pub lot_multiplier: usize,
    #[serde(rename = "optionPremium")]
    pub option_premium: f64,
}

impl MarginCalculatorPosition {
    /// Returns a new position
    pub fn new<T>(
        exchange: ExchangeType,
        product_type: ProductType,
        trade_type: TransactionType,
        token: T,
        price: f64,
        quantity: usize,
    ) -> Self
    where
        T: Into<String>,
    {
        Self {
            exchange,
            quantity,
            price,
            product_type,
            token: token.into(),
            trade_type,
        }
    }
}

impl MarginCalculatorReq {
    /// Returns a new instance for [`MarginCalculatorReq`]
    pub fn new() -> Self {
        Self { positions: vec![] }
    }

    /// Adds a position for margin calculation
    pub fn add_position(&mut self, position: MarginCalculatorPosition) -> &mut Self {
        self.positions.push(position);
        self
    }

    /// Adds positions for margin calculation
    pub fn add_positions<P>(&mut self, positions: P) -> &mut Self
    where
        P: AsRef<[MarginCalculatorPosition]>,
    {
        self.positions.extend_from_slice(positions.as_ref());
        self
    }
}
