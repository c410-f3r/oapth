#[cfg(feature = "dev-tools")]
mod clean;
mod migrate;
mod rollback;
#[cfg(feature = "dev-tools")]
mod seed;
mod validate;

use crate::BackEnd;

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
}
