//! Oapth - Commons

#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
mod macros;

mod database;
mod error;

#[cfg(feature = "std")]
mod migration_parser;
mod repeatability;
#[cfg(feature = "std")]
mod toml_parser;
#[cfg(feature = "std")]
mod utils;

pub use database::*;
pub use error::*;
#[cfg(feature = "std")]
pub use migration_parser::*;
pub use repeatability::*;
#[cfg(feature = "std")]
pub use utils::*;

/// core::result::Result<T, oapth_commons::Error>
pub type Result<T> = core::result::Result<T, Error>;
