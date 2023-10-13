//! Crate contains Angel One API SDK data structures and calls
#![forbid(unsafe_code)]
#![deny(unused_imports)]
#![deny(unused_variables)]
#![deny(missing_docs)]
#![deny(clippy::all)]

#[macro_use]
extern crate dtcm_angel_derive;

#[macro_use]
extern crate serde;

pub use dtcm_angel_utils::Result;

mod smart_connect;
pub use smart_connect::SmartConnect;

mod api;
pub use api::{funds, gtt, market, order, portfolio, user, ws};

/// Various types for Angel One API SDK
pub mod types;

#[cfg(test)]
pub mod test_util;
