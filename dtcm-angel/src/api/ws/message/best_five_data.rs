use dtcm_angel_utils::Error;
use serde_repr::Deserialize_repr;

/// Buy/Sell Flag
#[derive(Debug, Deserialize_repr)]
#[repr(i16)]
pub enum Flag {
    Sell = 0,
    Buy = 1,
}

#[derive(Debug, Deserialize)]
pub struct BestFiveData {
    /// Buy/Sell Flag
    pub flag: Flag,
    /// Quantity
    pub quantity: i64,
    /// Price
    pub price: i64,
    /// Number of Orders
    pub order_count: i16,
}

impl TryFrom<i16> for Flag {
    type Error = Error;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        let _self = match value {
            0 => Self::Sell,
            1 => Self::Buy,
            _ => return Err("Invalid best five data flag".into()),
        };
        Ok(_self)
    }
}
