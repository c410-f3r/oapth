//! oapth
//!
//! Flexible version control for databases through SQL migrations.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

#[macro_use]
mod utils;

oapth_macros::_any_db_! { mod fixed_sql_commands; }
#[cfg(all(feature = "_integration-tests", test))]
oapth_macros::_any_db_! { mod integration_tests; }
mod back_end_impls;
mod back_ends;
mod commands;
mod config;
pub mod doc_tests;
mod error;
mod migration;

#[oapth_macros::_diesel_mysql]
pub use back_end_impls::diesel::DieselMysql;
#[oapth_macros::_diesel_pg]
pub use back_end_impls::diesel::DieselPg;
#[oapth_macros::_diesel_sqlite]
pub use back_end_impls::diesel::DieselSqlite;
#[oapth_macros::_mysql_async]
pub use back_end_impls::mysql_async::MysqlAsync;
#[oapth_macros::_rusqlite]
pub use back_end_impls::rusqlite::Rusqlite;
#[oapth_macros::_sqlx_mssql]
pub use back_end_impls::sqlx::SqlxMssql;
#[oapth_macros::_sqlx_mysql]
pub use back_end_impls::sqlx::SqlxMysql;
#[oapth_macros::_sqlx_pg]
pub use back_end_impls::sqlx::SqlxPg;
#[oapth_macros::_sqlx_sqlite]
pub use back_end_impls::sqlx::SqlxSqlite;
#[oapth_macros::_tiberius]
pub use back_end_impls::tiberius::Tiberius;
#[oapth_macros::_tokio_postgres]
pub use back_end_impls::tokio_postgres::TokioPostgres;
pub use back_ends::back_end::BackEnd;
pub use commands::Commands;
pub use config::Config;
pub use error::Error;
pub use migration::{
  migration_group::{MigrationGroup, MigrationGroupOwned, MigrationGroupRef},
  Migration, MigrationOwned, MigrationRef,
};
#[oapth_macros::_embed_migrations]
pub use oapth_macros::embed_migrations;

use alloc::boxed::Box;
use back_ends::back_end_generic::BackEndGeneric;
use core::{future::Future, pin::Pin};
use migration::{
  db_migration::DbMigration,
  migration_common::{MigrationCommon, MigrationCommonOwned},
};
use utils::*;

/// Default batch size
pub const DEFAULT_BATCH_SIZE: usize = 128;
#[oapth_macros::_std]
/// Default environment variable name for the database URL
pub const DEFAULT_ENV_VAR: &str = "DATABASE_URL";

#[oapth_macros::_with_schema]
const OAPTH_SCHEMA_PREFIX: &str = "_oapth.";

/// Useful in constant environments where the type must be explicitly declared.
///
/// ```rust
/// const MIGRATIONS: EmbeddedMigrationsTy = embed_migrations!("SOME_CFG_FILE.toml");
/// ```
pub type EmbeddedMigrationsTy =
  &'static [(MigrationGroupRef<'static>, &'static [MigrationRef<'static, 'static>])];
/// Alias for `core::result::Result<T, oapth::Error>`
pub type Result<T> = core::result::Result<T, Error>;

type BoxFut<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;
