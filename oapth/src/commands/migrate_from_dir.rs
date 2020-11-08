use crate::{map_paths_into_migrations, scan_canonical_migrations_dir, Backend, Commands};
use std::path::Path;

impl<B> Commands<B>
where
  B: Backend,
{
  /// Applies `migrate` to a set of groups according to the canonical directory structure.
  #[inline]
  pub async fn migrate_from_dir<'a>(
    &'a mut self,
    dir: &'a Path,
    files_num: usize,
  ) -> crate::Result<()> {
    self.backend.create_oapth_tables().await?;
    let mut buffer = Vec::with_capacity(16);
    let mut all_migrations = scan_canonical_migrations_dir(dir)?;
    all_migrations.sort();
    for (mg, mut migrations_vec) in all_migrations {
      migrations_vec.sort();
      let mut migrations = map_paths_into_migrations(migrations_vec.into_iter());
      loop_files!(buffer, migrations, files_num, self.do_migrate(&mg, buffer.iter()).await?);
    }
    Ok(())
  }
}
