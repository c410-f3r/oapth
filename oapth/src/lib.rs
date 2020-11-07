//! oapth
//!
//! Flexible version control for databases through SQL migrations.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

#[macro_use]
mod utils;

mod backend;
mod commands;
mod config;
pub mod doc_tests;
mod error;
mod fixed_sql_commands;
mod migration;
#[cfg(feature = "std")]
mod parse_sql_file;

#[cfg(feature = "with-mysql_async")]
pub use backend::mysql_async::*;
#[cfg(feature = "with-rusqlite")]
pub use backend::rusqlite::*;
#[cfg(any(
  feature = "with-sqlx-mssql",
  feature = "with-sqlx-mysql",
  feature = "with-sqlx-postgres",
  feature = "with-sqlx-sqlite",
))]
pub use backend::sqlx::*;
#[cfg(feature = "with-tokio-postgres")]
pub use backend::tokio_postgres::*;
pub use backend::*;
pub use commands::*;
pub use config::*;
pub use error::*;
pub use migration::{migration_group::*, *};
#[cfg(feature = "std")]
pub use parse_sql_file::*;

use alloc::boxed::Box;
use core::{future::Future, pin::Pin};
use migration::{db_migration::*, migration_common::*, migration_params::*};
use utils::*;

const _OAPTH_SCHEMA: &str = "_oapth.";

/// Alias for `Pin<Box<dyn Future<Output = T> + 'a>>`
pub type BoxFut<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;
/// Alias for `core::result::Result<T, oapth::Error>`
pub type Result<T> = core::result::Result<T, Error>;
