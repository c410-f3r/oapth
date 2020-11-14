mod migrate;
#[cfg(feature = "dev-tools")]
mod recreate;
mod rollback;
#[cfg(feature = "dev-tools")]
mod seed;
mod validate;

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

  /// Returns the underlying backend
  #[inline]
  pub fn backend_mut(&mut self) -> &mut B {
    &mut self.backend
  }
}

#[cfg(all(feature = "_integration_tests", test))]
mod tests {
  use crate::{Backend, Commands};

  async fn test_all_commands<B>(c: &mut Commands<B>)
  where
    B: Backend,
  {
    crate::commands::rollback::tests::rollback_works(c).await;
    crate::commands::migrate::tests::migrate_works(c).await;
  }

  create_integration_tests!(test_all_commands);
}
