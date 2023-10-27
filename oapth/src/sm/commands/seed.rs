use crate::{database::Database, sm::Commands};
use alloc::string::String;
#[cfg(feature = "std")]
use std::{fs::read_to_string, path::Path};

impl<D> Commands<D>
where
  D: Database,
{
  /// Executes an arbitrary stream of SQL commands
  ///
  /// It is up to be caller to actually seed the database with data.
  #[inline]
  pub async fn seed<I, S>(&mut self, buffer_cmd: &mut String, seeds: I) -> crate::Result<()>
  where
    I: Iterator<Item = S>,
    S: AsRef<str>,
  {
    for elem in seeds {
      buffer_cmd.push_str(elem.as_ref());
    }
    self.database.transaction(buffer_cmd).await?;
    buffer_cmd.clear();
    Ok(())
  }

  /// Applies `Commands::seed` from a set of files located inside a given `dir`.
  #[cfg(feature = "std")]
  #[inline]
  pub async fn seed_from_dir(&mut self, buffer_cmd: &mut String, dir: &Path) -> crate::Result<()> {
    let iter = crate::sm::utils::files(dir)?.filter_map(|el_rslt| {
      let el = el_rslt.ok()?;
      read_to_string(el.path()).ok()
    });
    self.seed(buffer_cmd, iter).await
  }
}
