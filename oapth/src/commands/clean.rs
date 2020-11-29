use crate::{BackEnd, Commands};

impl<B> Commands<B>
where
  B: BackEnd
{
  /// Tries to clean all objects of a database, including separated namespaces/schemas.
  #[inline]
  pub async fn clean(&mut self) -> crate::Result<()> {
    Ok(self.back_end.clean().await?)
  }
}
