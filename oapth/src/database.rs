//! Database

#[cfg(any(feature = "sqlx-mysql", feature = "sqlx-postgres", feature = "sqlx-sqlite",))]
mod sqlx;
#[cfg(feature = "tiberius")]
mod tiberius;
mod unit;

use crate::{row::Row, DatabaseTy, FromRow};
use alloc::vec::Vec;
use core::future::Future;
#[cfg(any(feature = "sqlx-mysql", feature = "sqlx-postgres", feature = "sqlx-sqlite",))]
pub use sqlx::*;
#[cfg(feature = "tiberius")]
pub use tiberius::*;

/// Any backend that connects to a database
pub trait Database {
  /// See [DatabaseTy].
  const TY: DatabaseTy;

  /// See [Row].
  type Row: Row;

  /// Executes a raw SQL command.
  fn execute(&mut self, cmd: &str) -> impl Future<Output = crate::Result<()>>;

  /// Retrieves a raw database row.
  fn row(&mut self, cmd: &str) -> impl Future<Output = crate::Result<Self::Row>>;

  /// Retrieves a set of raw database rows.
  fn rows<E>(
    &mut self,
    cmd: &str,
    cb: impl FnMut(Self::Row) -> Result<(), E>,
  ) -> impl Future<Output = Result<(), E>>
  where
    E: From<crate::Error>;

  /// Retrieves a row and maps it to `T`. See [FromRow].
  #[inline]
  fn simple_entity<T>(&mut self, cmd: &str) -> impl Future<Output = Result<T, T::Error>>
  where
    T: FromRow<Self::Row>,
  {
    async move { T::from_row(&self.row(cmd).await?) }
  }

  /// Retrieves a set of rows and maps them to `T`. See [FromRow].
  #[inline]
  fn simple_entities<T>(
    &mut self,
    cmd: &str,
    results: &mut Vec<T>,
  ) -> impl Future<Output = Result<(), T::Error>>
  where
    T: FromRow<Self::Row>,
  {
    self.rows(cmd, |row| {
      results.push(T::from_row(&row)?);
      Ok::<_, T::Error>(())
    })
  }

  /// Similar to `[Self::execute]` but operations are atomics.
  fn transaction(&mut self, cmd: &str) -> impl Future<Output = crate::Result<()>>;
}
