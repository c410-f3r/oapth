oapth_macros::_dev_tools_! { mod clean; }
mod migrate;
mod rollback;
oapth_macros::_dev_tools_! { mod seed; }
mod validate;

use oapth_commons::Database;

use crate::{Backend, Migration, DEFAULT_BATCH_SIZE};

/// SQL commands facade
#[derive(Debug)]
pub struct Commands<B> {
  pub(crate) backend: B,
  #[allow(
    // An important part of the public interface but only used on std environments
    dead_code
  )]
  pub(crate) batch_size: usize,
}

impl<B> Commands<B>
where
  B: Backend,
{
  /// Creates a new instance from a given Backend and batch size.
  #[inline]
  pub fn new(backend: B, batch_size: usize) -> Self {
    Self { backend, batch_size }
  }

  /// Creates a new instance from a given Backend.
  ///
  /// Batch size will default to 128.
  #[inline]
  pub fn with_backend(backend: B) -> Self {
    let batch_size = DEFAULT_BATCH_SIZE;
    Self { backend, batch_size }
  }

  #[inline]
  fn filter_by_db<'migration, DBS, I, S>(
    migrations: I,
  ) -> impl Clone + Iterator<Item = &'migration Migration<DBS, S>>
  where
    DBS: AsRef<[Database]> + 'migration,
    I: Clone + Iterator<Item = &'migration Migration<DBS, S>>,
    S: AsRef<str> + 'migration,
  {
    let db = B::database();
    migrations.filter(move |m| if m.dbs().is_empty() { true } else { m.dbs().contains(&db) })
  }
}
