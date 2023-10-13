use dtcm_angel_utils::Error;
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Exchange type for subscription
#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum SubscriptionExchange {
    /// NSE Eq
    NSECM = 1,
    /// NSE FNO
    NSEFO = 2,
    /// BSE Eq
    BSECM = 3,
    /// BSE FNO
    BSEFO = 4,
    /// MCX FNO
    MCXFO = 5,
    /// NCX FNO
    NCXFO = 7,
    /// CDE FNO
    CDEFO = 13,
}

impl Default for SubscriptionExchange {
    fn default() -> Self {
        Self::NSECM
    }
}

impl TryFrom<u8> for SubscriptionExchange {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let _self = match value {
            1 => Self::NSECM,
            2 => Self::NSEFO,
            3 => Self::BSECM,
            4 => Self::BSEFO,
            5 => Self::MCXFO,
            7 => Self::NCXFO,
            13 => Self::CDEFO,
            _ => return Err("Invalid Subscription Exchange".into()),
        };
        Ok(_self)
    }
}
