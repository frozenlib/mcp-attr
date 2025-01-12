pub mod client;
mod common;
pub mod error;
#[doc(hidden)]
pub mod helpers;
pub mod schema;
mod schema_ext;
pub mod server;
mod transitivity;
pub mod utils;

pub use jsoncall;
pub use jsoncall::{Error, ErrorCode, Result, bail, bail_public};

const PROTOCOL_VERSION: &str = "2024-11-05";

#[cfg(doctest)]
mod tests_readme;
