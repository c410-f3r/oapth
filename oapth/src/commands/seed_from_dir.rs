use crate::{files, Backend, Commands};
use std::{fs::read_to_string, path::Path};

impl<B> Commands<B>
where
  B: Backend,
{
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
