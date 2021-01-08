//! Oapth - Commons

#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
mod macros;

mod database;
mod error;
#[cfg(feature = "std")]
mod parsers;
mod repeatability;
#[cfg(feature = "std")]
mod utils;

pub use database::Database;
pub use error::Error;
#[cfg(feature = "std")]
pub use parsers::{
  parse_migration::{parse_migration_cfg, parse_unified_migration},
  parse_root_cfg, parse_root_cfg_raw,
};
pub use repeatability::Repeatability;
#[cfg(feature = "std")]
pub use utils::{calc_checksum, files, group_and_migrations_from_path};

#[cfg(feature = "std")]
use utils::dir_name_parts;

/// core::result::Result<T, oapth_commons::Error>
pub type Result<T> = core::result::Result<T, Error>;
