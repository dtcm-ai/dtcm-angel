use std::io::Cursor;

use byteorder::{ReadBytesExt, LE};
use dtcm_angel_utils::Error;

use super::{best_five_data::Flag, BestFiveData};

/// Response for snap quote subscription request
#[derive(Debug, Deserialize)]
pub struct SnapQuote {
    /// Last traded timestamp
    pub last_traded_timestamp: i64,
    /// Open Interest
    pub open_interest: i64,
    /// Open Interest change % (this is a dummy field. contains garbage value)
    pub open_interest_change_percentage: f64,
    /// Best Five Data
    pub best_five_data: Vec<BestFiveData>,
    /// Upper circuit limit
    pub upper_circuit_limit: i64,
    /// Lower circuit limit
    pub lower_circuit_limit: i64,
    /// 52 week high price
    pub week_52_high_price: i64,
    /// 52 week low price
    pub week_52_low_price: i64,
}

impl TryFrom<&mut Cursor<&[u8]>> for SnapQuote {
    type Error = Error;

    fn try_from(rdr: &mut Cursor<&[u8]>) -> Result<Self, Self::Error> {
        let last_traded_timestamp = rdr.read_i64::<LE>()?;
        let open_interest = rdr.read_i64::<LE>()?;
        let open_interest_change_percentage = rdr.read_f64::<LE>()?;

        let mut best_five_data = vec![];
        for _ in 0..10 {
            best_five_data.push(BestFiveData {
                flag: Flag::try_from(rdr.read_i16::<LE>()?)?,
                quantity: rdr.read_i64::<LE>()?,
                price: rdr.read_i64::<LE>()?,
                order_count: rdr.read_i16::<LE>()?,
            });
        }

        Ok(Self {
            last_traded_timestamp,
            open_interest,
            open_interest_change_percentage,
            best_five_data,
            upper_circuit_limit: rdr.read_i64::<LE>()?,
            lower_circuit_limit: rdr.read_i64::<LE>()?,
            week_52_high_price: rdr.read_i64::<LE>()?,
            week_52_low_price: rdr.read_i64::<LE>()?,
        })
    }
}
