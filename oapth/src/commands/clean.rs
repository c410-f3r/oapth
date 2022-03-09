use crate::{Backend, Commands};
use alloc::string::String;

impl<B> Commands<B>
where
  B: Backend,
{
  /// Tries to clean all objects of a database, including separated namespaces/schemas.
  #[inline]
  pub async fn clean(&mut self, buffer: &mut String) -> crate::Result<()> {
    self.backend.clean(buffer).await
  }
}
