#[cfg(feature = "sm-dev")]
mod clear;
mod migrate;
mod rollback;
#[cfg(feature = "sm-dev")]
mod seed;
mod validate;

use crate::{
  database::Database,
  sm::{UserMigration, DEFAULT_BATCH_SIZE},
  DatabaseTy,
};

/// SQL commands facade
#[derive(Debug)]
pub struct Commands<D> {
  batch_size: usize,
  pub(crate) database: D,
}

impl<D> Commands<D>
where
  D: Database,
{
  /// Creates a new instance from a given Backend and batch size.
  #[inline]
  pub fn new(batch_size: usize, database: D) -> Self {
    Self { batch_size, database }
  }

  /// Creates a new instance from a given Backend.
  ///
  /// Batch size will default to 128.
  #[inline]
  pub fn with_database(database: D) -> Self {
    Self { batch_size: DEFAULT_BATCH_SIZE, database }
  }

  /// Batch size
  #[inline]
  pub fn batch_size(&self) -> usize {
    self.batch_size
  }

  #[inline]
  fn filter_by_db<'migration, DBS, I, S>(
    migrations: I,
  ) -> impl Clone + Iterator<Item = &'migration UserMigration<DBS, S>>
  where
    DBS: AsRef<[DatabaseTy]> + 'migration,
    I: Clone + Iterator<Item = &'migration UserMigration<DBS, S>>,
    S: AsRef<str> + 'migration,
  {
    migrations.filter(move |m| if m.dbs().is_empty() { true } else { m.dbs().contains(&D::TY) })
  }
}
