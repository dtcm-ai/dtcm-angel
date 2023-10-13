use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Deserializer, Serializer};

use crate::Result;

/// Converts yyyy-mm-dd hh:mm e.g. 2001-07-08 00:34 to the NaiveDateTime
pub fn from_yyyy_mm_dd_hh_mm<D>(date_str: D) -> Result<NaiveDateTime>
where
    D: AsRef<str>,
{
    NaiveDateTime::parse_from_str(date_str.as_ref(), "%F %R").map_err(|e| e.into())
}

/// Serializes the NaiveDateTime to yyyy-mm-dd hh:mm e.g. 2001-07-08 00:34
pub fn serde_yyyy_mm_dd_hh_mm<S>(
    date: &NaiveDateTime,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = format!("{}", date.format("%F %R"));
    serializer.serialize_str(&s)
}

/// Deserializes dates in the format of ddMMyyyy e.g. 28Oct2023 to optional NaiveDate
#[allow(non_snake_case)]
pub fn serde_ddMMyyyy<'de, D>(deserializer: D) -> std::result::Result<Option<NaiveDate>, D::Error>
where
    D: Deserializer<'de>,
{
    String::deserialize(deserializer).map(|s| chrono::NaiveDate::parse_from_str(&s, "%d%b%Y").ok())
}
