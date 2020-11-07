use crate::{map_paths_into_migrations, scan_canonical_migrations_dir, Backend, Commands};
use std::path::Path;

impl<B> Commands<B>
where
  B: Backend,
{
  /// Applies `validate` to a set of groups according to the canonical directory structure.
  #[inline]
  pub async fn validate_from_dir<'a>(
    &'a mut self,
    dir: &'a Path,
    files_num: usize,
  ) -> crate::Result<()> {
    let mut buffer = Vec::with_capacity(16);
    for (mg, migrations_vec) in scan_canonical_migrations_dir(dir)? {
      let mut migrations = map_paths_into_migrations(migrations_vec.into_iter());
      loop_files!(buffer, migrations, files_num, self.validate(&mg, buffer.iter()).await?);
    }
    Ok(())
  }
}
