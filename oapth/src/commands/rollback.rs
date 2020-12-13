use crate::{BackEnd, Commands, Migration, MigrationGroup};
#[oapth_macros::std_]
use {
  crate::{group_and_migrations_from_path, parse_root_cfg},
  std::{fs::File, path::Path},
};

impl<B> Commands<B>
where
  B: BackEnd,
{
  /// Rollbacks the migrations of a group to a given `version`.
  ///
  /// Before issuing a rollback, all migrations are validated.
  #[inline]
  pub async fn rollback<'a, I>(
    &'a mut self,
    mg: &MigrationGroup,
    migrations: I,
    version: i32,
  ) -> crate::Result<()>
  where
    I: Clone + DoubleEndedIterator<Item = &'a Migration> + 'a,
  {
    let db_migrations = self.back_end.migrations(mg).await?;
    let filtered_by_db = Self::filter_by_db(migrations);
    Self::do_validate(&db_migrations, filtered_by_db.clone())?;
    let reverts_iter = filtered_by_db.map(|el| el.sql_down());
    self.back_end.transaction(reverts_iter).await?;
    self.back_end.delete_migrations(version, mg).await?;
    Ok(())
  }

  #[allow(
    // Sortable elements are not copyable
    clippy::unnecessary_sort_by
  )]
  /// Applies `rollback` to a set of groups according to the configuration file
  #[inline]
  #[oapth_macros::std_]
  pub async fn rollback_from_cfg<'a>(
    &'a mut self,
    path: &'a Path,
    versions: &'a [i32],
    files_num: usize,
  ) -> crate::Result<()> {
    let cfg_dir = path.parent().unwrap_or_else(|| Path::new("."));
    let mut dirs_str = parse_root_cfg(File::open(path)?, cfg_dir)?;
    if dirs_str.len() != versions.len() {
      return Err(crate::Error::DifferentRollbackVersions);
    }
    dirs_str.sort_by(|a, b| b.cmp(a));
    let mut buffer = Vec::with_capacity(16);
    for (dir_str, &version) in dirs_str.into_iter().zip(versions) {
      self.do_rollback_from_dir(&mut buffer, &dir_str, version, files_num).await?;
    }
    Ok(())
  }

  /// Applies `rollback` to a set of migrations according to a given directory
  #[inline]
  #[oapth_macros::std_]
  pub async fn rollback_from_dir<'a>(
    &'a mut self,
    path: &'a Path,
    version: i32,
    files_num: usize,
  ) -> crate::Result<()> {
    let mut buffer = Vec::with_capacity(16);
    self.do_rollback_from_dir(&mut buffer, path, version, files_num).await
  }

  #[inline]
  #[oapth_macros::std_]
  async fn do_rollback_from_dir<'a>(
    &'a mut self,
    buffer: &'a mut Vec<Migration>,
    path: &'a Path,
    version: i32,
    files_num: usize,
  ) -> crate::Result<()> {
    let opt = group_and_migrations_from_path(path, |a, b| b.cmp(a));
    let (mg, mut migrations) = if let Some(rslt) = opt { rslt } else { return Ok(()) };
    loop_files!(buffer, migrations, files_num, self.rollback(&mg, buffer.iter(), version).await?);
    Ok(())
  }
}
