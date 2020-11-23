//! oapth
//!
//! Flexible version control for databases through SQL migrations.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

#[macro_use]
mod utils;

mod _back_end;
mod back_end;
mod commands;
mod config;
pub mod doc_tests;
mod error;
mod fixed_sql_commands;
#[cfg(all(feature = "_integration-tests", test))]
mod integration_tests;
mod migration;
#[cfg(feature = "std")]
mod parsers;

#[cfg(feature = "with-diesel-mysql")]
pub use back_end::diesel::DieselMysql;
#[cfg(feature = "with-diesel-postgres")]
pub use back_end::diesel::DieselPostgres;
#[cfg(feature = "with-diesel-sqlite")]
pub use back_end::diesel::DieselSqlite;
#[cfg(feature = "with-mysql_async")]
pub use back_end::mysql_async::MysqlAsync;
#[cfg(feature = "with-rusqlite")]
pub use back_end::rusqlite::Rusqlite;
#[cfg(feature = "with-sqlx-mssql")]
pub use back_end::sqlx::SqlxMssql;
#[cfg(feature = "with-sqlx-mysql")]
pub use back_end::sqlx::SqlxMysql;
#[cfg(feature = "with-sqlx-postgres")]
pub use back_end::sqlx::SqlxPostgres;
#[cfg(feature = "with-sqlx-sqlite")]
pub use back_end::sqlx::SqlxSqlite;
#[cfg(feature = "with-tiberius")]
pub use back_end::tiberius::Tiberius;
#[cfg(feature = "with-tokio-postgres")]
pub use back_end::tokio_postgres::TokioPostgres;
pub use back_end::BackEnd;
pub use commands::Commands;
pub use config::Config;
pub use error::Error;
pub use migration::{migration_group::MigrationGroup, Migration};
#[cfg(feature = "std")]
pub use parsers::{parse_cfg, parse_migration};

use _back_end::_BackEnd;
use alloc::boxed::Box;
use core::{future::Future, pin::Pin};
use migration::{
  db_migration::DbMigration, migration_common::MigrationCommon, migration_params::MigrationParams,
};
use utils::*;

const _OAPTH_SCHEMA_PREFIX: &str = "_oapth.";

type BoxFut<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;
/// Alias for `core::result::Result<T, oapth::Error>`
pub type Result<T> = core::result::Result<T, Error>;
