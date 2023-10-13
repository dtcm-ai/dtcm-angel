//! Crate contains utility calls for the SDK, usable types are reexported from dtcm-angel crate
#![forbid(unsafe_code)]
#![deny(unused_imports)]
#![deny(unused_variables)]
#![deny(missing_docs)]
#![deny(clippy::all)]

#[macro_use]
extern crate pin_project;

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde;

/// Type alias for Result
/// Type alias for error
pub type Error = Box<dyn std::error::Error>;

/// Type alias for result
pub type Result<T> = std::result::Result<T, Error>;

/// Contains date related functionality
pub mod date;
/// Contains http related functionality
pub mod http;
/// Contains system related functionality
pub mod sys;
/// Contains websocket related functionality
pub mod ws;
