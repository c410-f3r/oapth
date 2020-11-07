use crate::{map_paths_into_migrations, scan_canonical_migrations_dir, Backend, Commands};
use std::path::Path;

impl<B> Commands<B>
where
  B: Backend,
{
  /// Applies `rollback` to a set of groups according to the canonical directory structure.
  #[inline]
  pub async fn rollback_from_dir<'a, I>(
    &'a mut self,
    dir: &'a Path,
    versions: I,
    files_num: usize,
  ) -> crate::Result<()>
  where
    I: Iterator<Item = i32>,
  {
    let mut buffer = Vec::with_capacity(16);
    for (rollback_version, (mg, migrations_vec)) in
      versions.zip(scan_canonical_migrations_dir(dir)?)
    {
      let mut migrations = map_paths_into_migrations(migrations_vec.into_iter());
      loop_files!(
        buffer,
        migrations,
        files_num,
        self.rollback(&mg, buffer.iter(), rollback_version).await?
      );
    }
    Ok(())
  }
}
