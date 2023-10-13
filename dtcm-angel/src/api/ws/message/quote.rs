use std::io::Cursor;

use byteorder::{ReadBytesExt, LE};
use dtcm_angel_utils::Error;

/// Response for quote subscription request
#[derive(Debug, Deserialize)]
pub struct Quote {
    /// Last traded quantity
    pub last_traded_quantity: i64,
    /// Average traded price
    pub average_traded_price: i64,
    /// Volume traded for the day
    pub volume_trade_for_the_day: i64,
    /// Total buy quantity
    pub total_buy_quantity: f64,
    /// Total sell quantity
    pub total_sell_quantity: f64,
    /// Open price of the day
    pub open_price_of_the_day: i64,
    /// High price of the day
    pub high_price_of_the_day: i64,
    /// Low price of the day
    pub low_price_of_the_day: i64,
    /// Close price
    pub closed_price: i64,
}

impl TryFrom<&mut Cursor<&[u8]>> for Quote {
    type Error = Error;

    fn try_from(rdr: &mut Cursor<&[u8]>) -> Result<Self, Self::Error> {
        Ok(Self {
            last_traded_quantity: rdr.read_i64::<LE>()?,
            average_traded_price: rdr.read_i64::<LE>()?,
            volume_trade_for_the_day: rdr.read_i64::<LE>()?,
            total_buy_quantity: rdr.read_f64::<LE>()?,
            total_sell_quantity: rdr.read_f64::<LE>()?,
            open_price_of_the_day: rdr.read_i64::<LE>()?,
            high_price_of_the_day: rdr.read_i64::<LE>()?,
            low_price_of_the_day: rdr.read_i64::<LE>()?,
            closed_price: rdr.read_i64::<LE>()?,
        })
    }
}
