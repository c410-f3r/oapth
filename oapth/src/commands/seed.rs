use crate::{Backend, Commands};
#[oapth_macros::_std]
use {
  std::{fs::read_to_string, path::Path},
};

impl<B> Commands<B>
where
  B: Backend,
{
  /// Executes an arbitrary stream of SQL commands
  ///
  /// It is up to be caller to actually seed the database with data.
  #[inline]
  pub async fn seed<I, S>(&mut self, seeds: I) -> crate::Result<()>
  where
    I: Iterator<Item = S> + Send,
    S: AsRef<str> + Send + Sync
  {
    self.backend.transaction(seeds).await?;
    Ok(())
  }

  /// Applies `Commands::seed` from a set of files located inside a given `dir`.
  #[oapth_macros::_std]
  #[inline]
  pub async fn seed_from_dir(&mut self, dir: &Path) -> crate::Result<()> {
    let iter = oapth_commons::files(dir)?.filter_map(|el_rslt| {
      let el = el_rslt.ok()?;
      read_to_string(el.path()).ok()
    });
    self.seed(iter).await
  }
}
