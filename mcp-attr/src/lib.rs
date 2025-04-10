pub mod client;
mod common;
#[doc(hidden)]
pub mod helpers;

/// Types defined in the [MCP protocol schema]
///
/// This module was automatically generated from the [MCP protocol schema].
///
/// [MCP protocol schema]: https://github.com/modelcontextprotocol/specification/blob/main/schema/2024-11-05/schema.json
pub mod schema;
mod schema_ext;
pub mod server;
mod transitivity;
pub mod utils;

pub use jsoncall;
pub use jsoncall::{Error, ErrorCode, Result, SessionError, SessionResult, bail, bail_public};

#[cfg(doctest)]
mod tests_readme;
