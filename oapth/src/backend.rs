#[cfg(feature = "with-mysql_async")]
pub(crate) mod mysql_async;
#[cfg(feature = "with-rusqlite")]
pub(crate) mod rusqlite;
#[cfg(any(
  feature = "with-sqlx-mssql",
  feature = "with-sqlx-mysql",
  feature = "with-sqlx-postgres",
  feature = "with-sqlx-sqlite",
))]
pub(crate) mod sqlx;
#[cfg(feature = "with-tokio-postgres")]
pub(crate) mod tokio_postgres;
pub(crate) mod unit;

use crate::{BoxFut, DbMigration, Migration, MigrationGroup};
use alloc::vec::Vec;

/// A back end is the bridge between Rust and a database.
pub trait Backend {
  /// Creates the necessary internal tables
  fn create_oapth_tables<'a>(&'a mut self) -> BoxFut<'a, crate::Result<()>>;

  /// Executes arbitrary DDL or DML
  fn execute<'a>(&'a mut self, command: &'a str) -> BoxFut<'a, crate::Result<()>>;

  /// Inserts a stream of migrations of a given group.
  fn insert_migrations<'a, I>(
    &'a mut self,
    migrations: I,
    mg: &'a MigrationGroup,
  ) -> BoxFut<'a, crate::Result<()>>
  where
    I: Clone + Iterator<Item = &'a Migration> + 'a;

  /// Returns a vector with all migrations of a group within the database.
  fn migrations<'a>(
    &'a mut self,
    mg: &'a MigrationGroup,
  ) -> BoxFut<'a, crate::Result<Vec<DbMigration>>>;

  /// Executes a stream of arbitrary DDL or DML inside a transaction.
  fn transaction<'a, I, S>(&'a mut self, commands: I) -> BoxFut<'a, crate::Result<()>>
  where
    I: Iterator<Item = S> + 'a,
    S: AsRef<str>;
}
