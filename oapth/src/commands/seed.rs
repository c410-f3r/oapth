use crate::{BackEnd, Commands};
#[oapth_macros::std_]
use {
  crate::files,
  std::{fs::read_to_string, path::Path},
};
impl<B> Commands<B>
where
  B: BackEnd,
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
    self.back_end.transaction(seeds).await?;
    Ok(())
  }

  #[oapth_macros::std_]
  /// Applies `Commands::seed` from a set of files located inside a given `dir`.
  #[inline]
  pub async fn seed_from_dir<'a>(&'a mut self, dir: &'a Path) -> crate::Result<()> {
    let iter = files(dir)?.filter_map(|el_rslt| {
      let el = el_rslt.ok()?;
      Some(read_to_string(el.path()).ok()?)
    });
    self.seed(iter).await
  }
}
