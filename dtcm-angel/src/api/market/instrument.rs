use chrono::NaiveDate;
use dtcm_angel_utils::date::serde_ddMMyyyy;

use crate::types::ExchangeType;

/// Instrument record received from Angel One JSON URL
#[derive(Debug, Deserialize)]
pub struct Instrument {
    /// Token
    pub token: String,
    /// Symbol name
    pub symbol: String,
    /// Name
    pub name: String,
    /// Expiry
    #[serde(default, deserialize_with = "serde_ddMMyyyy")]
    pub expiry: Option<NaiveDate>,
    /// Strike
    pub strike: String,
    /// Lot size
    #[serde(rename = "lotsize")]
    pub lot_size: String,
    /// Instrument type
    #[serde(rename = "instrumenttype")]
    pub instrument_type: String,
    /// Exchange
    pub exch_seg: ExchangeType,
    /// Tick size
    pub tick_size: String,
}
