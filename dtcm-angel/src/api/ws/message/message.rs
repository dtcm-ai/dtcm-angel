use std::io::{Cursor, Read};

use byteorder::{ReadBytesExt, LE};
use serde::Deserialize;

use crate::ws::{SubscriptionExchange, SubscriptionMode};

use super::{Quote, SnapQuote};

/// Data response received from websocket server as binary message
#[derive(Debug, Deserialize)]
pub struct Message {
    /// Subscription Mode
    pub mode: SubscriptionMode,
    /// Exchange Type
    pub exchange: SubscriptionExchange,
    /// Symbol token
    pub token: String,
    /// Sequence Number
    pub sequence_number: i64,
    /// Exchange Timestamp
    pub exchange_timestamp: i64,
    /// Last Traded Price
    pub last_traded_price: i64,
    /// Response include with Quote Mode
    pub quote: Option<Quote>,
    /// Response include with Snap Quote Mode
    pub snap_quote: Option<SnapQuote>,
}

impl TryFrom<&[u8]> for Message {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut rdr = Cursor::new(value);

        let mode = SubscriptionMode::try_from(rdr.read_u8()?)?;
        let exchange = rdr.read_u8()?;

        let mut token = [0u8; 25];
        rdr.read_exact(&mut token)?;

        let token = std::str::from_utf8(&token)?
            .trim_matches(char::from(0))
            .to_string();

        let sequence_number = rdr.read_i64::<LE>()?;
        let exchange_timestamp = rdr.read_i64::<LE>()?;
        let last_traded_price = rdr.read_i64::<LE>()?;

        let quote = match mode {
            SubscriptionMode::Ltp => None,
            _ => Quote::try_from(&mut rdr).ok(),
        };

        let snap_quote = match mode {
            SubscriptionMode::SnapQuote => SnapQuote::try_from(&mut rdr).ok(),
            _ => None,
        };

        Ok(Message {
            mode,
            exchange: SubscriptionExchange::try_from(exchange)?,
            token,
            sequence_number,
            exchange_timestamp,
            last_traded_price,
            quote,
            snap_quote,
        })
    }
}

impl TryFrom<Vec<u8>> for Message {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(value.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use crate::ws::{SubscriptionExchange, SubscriptionMode};

    use super::Message;

    #[test]
    fn deserialize_snap_quote_works() {
        let bytes = [
            3, 1, 49, 48, 54, 50, 54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            227, 149, 233, 0, 0, 0, 0, 0, 125, 175, 44, 150, 138, 1, 0, 0, 136, 7, 2, 0, 0, 0, 0,
            0, 5, 0, 0, 0, 0, 0, 0, 0, 136, 7, 2, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 160, 97, 64, 0, 0, 0, 0, 0, 0, 78, 64, 136, 7, 2, 0, 0, 0, 0, 0, 136, 7, 2, 0, 0,
            0, 0, 0, 136, 7, 2, 0, 0, 0, 0, 0, 184, 4, 2, 0, 0, 0, 0, 0, 208, 107, 160, 98, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 20, 0, 0, 0, 0, 0, 0, 0,
            124, 9, 2, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 136, 7, 2, 0, 0, 0, 0, 0,
            1, 0, 1, 0, 50, 0, 0, 0, 0, 0, 0, 0, 142, 168, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 50, 0, 0,
            0, 0, 0, 0, 0, 141, 168, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 20, 0, 0, 0, 0, 0, 0, 0, 244,
            165, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 91, 11, 2, 0, 0, 0, 0, 0,
            1, 0, 0, 0, 50, 0, 0, 0, 0, 0, 0, 0, 92, 11, 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 54, 0, 0, 0,
            0, 0, 0, 0, 92, 11, 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 112, 111,
            2, 0, 0, 0, 0, 0, 160, 159, 1, 0, 0, 0, 0, 0, 136, 7, 2, 0, 0, 0, 0, 0, 44, 145, 1, 0,
            0, 0, 0, 0,
        ];

        let m = Message::try_from(&bytes[..]).unwrap();
        assert_eq!(m.mode, SubscriptionMode::SnapQuote);
        assert_eq!(m.exchange, SubscriptionExchange::NSECM);
        assert_eq!(m.token, "10626");
        println!("{:?}", m);
    }
}
