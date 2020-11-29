//! oapth
//!
//! Flexible version control for databases through SQL migrations.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

#[macro_use]
mod utils;

oapth_macros::any_db! { mod fixed_sql_commands; }
#[cfg(all(feature = "_integration-tests", test))]
oapth_macros::any_db! { mod integration_tests; }
mod back_end_impls;
mod back_ends;
mod commands;
mod config;
mod database;
pub mod doc_tests;
mod error;
mod migration;
oapth_macros::std! { mod parsers; }

#[oapth_macros::diesel_mysql_]
pub use back_end_impls::diesel::DieselMysql;
#[oapth_macros::diesel_pg_]
pub use back_end_impls::diesel::DieselPg;
#[oapth_macros::diesel_sqlite_]
pub use back_end_impls::diesel::DieselSqlite;
#[oapth_macros::mysql_async_]
pub use back_end_impls::mysql_async::MysqlAsync;
#[oapth_macros::rusqlite_]
pub use back_end_impls::rusqlite::Rusqlite;
#[oapth_macros::sqlx_mssql_]
pub use back_end_impls::sqlx::SqlxMssql;
#[oapth_macros::sqlx_mysql_]
pub use back_end_impls::sqlx::SqlxMysql;
#[oapth_macros::sqlx_pg_]
pub use back_end_impls::sqlx::SqlxPg;
#[oapth_macros::sqlx_sqlite_]
pub use back_end_impls::sqlx::SqlxSqlite;
#[oapth_macros::tiberius_]
pub use back_end_impls::tiberius::Tiberius;
#[oapth_macros::tokio_postgres_]
pub use back_end_impls::tokio_postgres::TokioPostgres;
pub use back_ends::back_end::BackEnd;
pub use commands::Commands;
pub use config::Config;
pub use database::Database;
pub use error::Error;
pub use migration::{migration_group::MigrationGroup, Migration};

#[oapth_macros::std_]
pub use parsers::{parse_cfg, parse_migration::parse_migration};

use alloc::boxed::Box;
use arrayvec::ArrayVec;
use back_ends::back_end_generic::BackEndGeneric;
use core::{future::Future, pin::Pin};
use migration::{db_migration::DbMigration, migration_common::MigrationCommon};
use utils::*;

#[oapth_macros::with_schema_]
const OAPTH_SCHEMA_PREFIX: &str = "_oapth.";

type BoxFut<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;
type Dbs = ArrayVec<[Database; 4]>;
/// Alias for `core::result::Result<T, oapth::Error>`
pub type Result<T> = core::result::Result<T, Error>;
