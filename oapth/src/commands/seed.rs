use crate::{Backend, Commands};

impl<B> Commands<B>
where
  B: Backend,
{
  /// Executes an arbitrary stream of SQL commands
  ///
  /// It is up to be caller to actually seed the database with data.
  #[inline]
  pub async fn seed<'a, I, S>(&'a mut self, seeds: I) -> crate::Result<()>
  where
    I: Iterator<Item = S> + 'a,
    S: AsRef<str>,
  {
    self.backend.transaction(seeds).await?;
    Ok(())
  }
}
