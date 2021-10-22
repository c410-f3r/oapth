oapth_macros::_dev_tools_! { mod clean; }
mod migrate;
mod rollback;
oapth_macros::_dev_tools_! { mod seed; }
mod validate;

use crate::{BackEnd, MigrationRef, DEFAULT_BATCH_SIZE};

/// SQL commands facade
#[derive(Debug)]
pub struct Commands<B> {
  pub(crate) back_end: B,
  #[allow(
    // An important part of the public interface but only used on std environments
    dead_code
  )]
  pub(crate) batch_size: usize,
}

impl<B> Commands<B>
where
  B: BackEnd,
{
  /// Creates a new instance from a given BackEnd and batch size.
  #[inline]
  pub fn new(back_end: B, batch_size: usize) -> Self {
    Self { back_end, batch_size }
  }

  /// Creates a new instance from a given BackEnd.
  ///
  /// Batch size will default to 128.
  #[inline]
  pub fn with_back_end(back_end: B) -> Self {
    let batch_size = DEFAULT_BATCH_SIZE;
    Self { back_end, batch_size }
  }

  #[inline]
  fn filter_by_db<'a, 'b, I>(
    migrations: I,
  ) -> impl Clone + Iterator<Item = MigrationRef<'a, 'a>> + 'b
  where
    'a: 'b,
    I: Clone + Iterator<Item = MigrationRef<'a, 'a>> + 'b,
  {
    let db = B::database();
    migrations.filter(move |m| if m.dbs().is_empty() { true } else { m.dbs().contains(&db) })
  }
}
