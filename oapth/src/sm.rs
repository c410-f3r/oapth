//! Schema Management

#[macro_use]
mod macros;

mod commands;
pub mod doc_tests;
pub(crate) mod fixed_sql_commands;
mod migration;
#[cfg(feature = "std")]
pub mod migration_parser;
mod repeatability;
#[cfg(feature = "std")]
pub mod toml_parser;
pub mod utils;

pub use commands::*;
pub use repeatability::Repeatability;
#[cfg(all(feature = "_integration-tests", test))]
mod integration_tests;
use crate::{database::Database, DatabaseTy, Identifier};
use alloc::{string::String, vec::Vec};
use core::future::Future;
pub use migration::*;

/// Default batch size
pub const DEFAULT_BATCH_SIZE: usize = 128;
pub(crate) const _OAPTH: &str = "oapth";
pub(crate) const _OAPTH_SCHEMA_PREFIX: &str = "_oapth.";

/// Useful in constant environments where the type must be explicitly declared.
///
/// ```ignore,rust
/// const MIGRATIONS: EmbeddedMigrationsTy = embed_migrations!("SOME_CFG_FILE.toml");
/// ```
pub type EmbeddedMigrationsTy = &'static [(
  &'static MigrationGroup<&'static str>,
  &'static [UserMigrationRef<'static, 'static>],
)];

/// Contains methods responsible to manage database migrations.
pub trait SchemaManagement: Database {
  /// Clears all database resources.
  fn clear(
    &mut self,
    buffer: (&mut String, &mut Vec<Identifier>),
  ) -> impl Future<Output = crate::Result<()>>;

  /// Initial tables meant for initialization.
  fn create_oapth_tables(&mut self) -> impl Future<Output = crate::Result<()>>;

  /// Removes every migration of a given group `mg`` that is greater than `version`.
  fn delete_migrations<S>(
    &mut self,
    buffer_cmd: &mut String,
    mg: &MigrationGroup<S>,
    version: i32,
  ) -> impl Future<Output = crate::Result<()>>
  where
    S: AsRef<str>;

  /// Inserts a new set of migrations,
  fn insert_migrations<'migration, DBS, I, S>(
    &mut self,
    buffer_cmd: &mut String,
    mg: &MigrationGroup<S>,
    migrations: I,
  ) -> impl Future<Output = crate::Result<()>>
  where
    DBS: AsRef<[DatabaseTy]> + 'migration,
    I: Clone + Iterator<Item = &'migration UserMigration<DBS, S>>,
    S: AsRef<str> + 'migration;

  /// Retrieves all migrations of the given `mg` group.
  fn migrations<S>(
    &mut self,
    buffer_cmd: &mut String,
    mg: &MigrationGroup<S>,
    results: &mut Vec<DbMigration>,
  ) -> impl Future<Output = crate::Result<()>>
  where
    S: AsRef<str>;

  /// Retrieves all tables contained in a schema. If the implementation does not supports schemas,
  /// the parameter is ignored.
  fn table_names(
    &mut self,
    buffer_cmd: &mut String,
    results: &mut Vec<Identifier>,
    schema: &str,
  ) -> impl Future<Output = crate::Result<()>>;
}
