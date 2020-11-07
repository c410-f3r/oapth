mod migrate;
#[cfg(feature = "std")]
mod migrate_from_dir;
#[cfg(feature = "dev-tools")]
mod reset;
mod rollback;
#[cfg(feature = "std")]
mod rollback_from_dir;
#[cfg(feature = "dev-tools")]
mod seed;
#[cfg(all(feature = "dev-tools", feature = "std"))]
mod seed_from_dir;
mod validate;
#[cfg(feature = "std")]
mod validate_from_dir;

use crate::Backend;

/// SQL commands facade
#[derive(Debug)]
pub struct Commands<B> {
  backend: B,
}

impl<B> Commands<B>
where
  B: Backend,
{
  /// Creates a new instance from a given backend.
  #[inline]
  pub fn new(backend: B) -> Self {
    Self { backend }
  }
}
