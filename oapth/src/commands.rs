oapth_macros::dev_tools! { mod clean; }
mod migrate;
mod rollback;
oapth_macros::dev_tools! { mod seed; }
mod validate;

use crate::{BackEnd, Migration};

/// SQL commands facade
#[derive(Debug)]
pub struct Commands<B> {
  pub(crate) back_end: B,
}

impl<B> Commands<B>
where
  B: BackEnd,
{
  /// Creates a new instance from a given BackEnd.
  #[inline]
  pub fn new(back_end: B) -> Self {
    Self { back_end }
  }

  fn filter_by_db<'a, I>(migrations: I) -> impl Clone + Iterator<Item = &'a Migration>
  where
    I: Clone + Iterator<Item = &'a Migration>,
  {
    migrations.filter(|m| if m.dbs().is_empty() { true } else { m.dbs().contains(&B::database()) })
  }
}
