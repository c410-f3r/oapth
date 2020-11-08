use crate::{map_paths_into_migrations, scan_canonical_migrations_dir, Backend, Commands};
use std::path::Path;

impl<B> Commands<B>
where
  B: Backend,
{
  /// Applies `rollback` to a set of groups according to the canonical directory structure.
  #[allow(
    // Sortable elements are not copyable
    clippy::unnecessary_sort_by
  )]
  #[inline]
  pub async fn rollback_from_dir<'a, I>(
    &'a mut self,
    dir: &'a Path,
    versions: I,
    files_num: usize,
  ) -> crate::Result<()>
  where
    I: ExactSizeIterator + DoubleEndedIterator<Item = i32>,
  {
    let mut buffer = Vec::with_capacity(16);
    let mut all_migrations = scan_canonical_migrations_dir(dir)?;
    all_migrations.sort_by(|a, b| b.cmp(a));
    for (rollback_version, (mg, mut migrations_vec)) in versions.zip(all_migrations) {
      migrations_vec.sort_by(|a, b| b.cmp(a));
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
