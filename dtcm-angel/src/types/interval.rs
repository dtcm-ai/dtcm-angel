use chrono::NaiveDateTime;

/// Interval
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Interval {
    /// 1 Minute
    #[serde(rename = "ONE_MINUTE")]
    _1m,
    /// 3 Minute
    #[serde(rename = "THREE_MINUTE")]
    _3m,
    /// 5 Minute
    #[serde(rename = "FIVE_MINUTE")]
    _5m,
    /// 10 Minute
    #[serde(rename = "TEN_MINUTE")]
    _10m,
    /// 15 Minute
    #[serde(rename = "FIFTEEN_MINUTE")]
    _15m,
    /// 30 Minute
    #[serde(rename = "THIRTY_MINUTE")]
    _30m,
    /// 1 Hour
    #[serde(rename = "ONE_HOUR")]
    _1h,
    /// 1 Day
    #[serde(rename = "ONE_DAY")]
    _1d,
}

impl Interval {
    /// Returns the limit on days to fetch the data from the API
    pub fn limit(&self) -> i64 {
        match self {
            Self::_1m => 30,
            Self::_3m | Self::_5m | Self::_10m => 90,
            Self::_15m | Self::_30m => 180,
            Self::_1h => 365,
            Self::_1d => 2000,
        }
    }

    /// Validates the days limit for the interval for the given dates
    pub fn valid(&self, from_date: NaiveDateTime, to_date: NaiveDateTime) -> crate::Result<()> {
        let days = (to_date - from_date).num_days();
        if days > self.limit() {
            return Err(format!(
                "Only {} days allowed for {:?} only interval",
                self.limit(),
                self
            )
            .into());
        }

        Ok(())
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self::_1d
    }
}
