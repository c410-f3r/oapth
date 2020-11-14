use crate::{Backend, Commands, Migration, MigrationGroup};
#[cfg(feature = "std")]
use {
  crate::{group_and_migrations_from_path, parse_cfg},
  std::{fs::File, path::Path},
};

impl<B> Commands<B>
where
  B: Backend,
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
    let db_migrations = self.backend.migrations(mg).await?;
    self.do_validate(&db_migrations, migrations.clone())?;
    let reverts_iter = migrations.map(|el| el.sql_down());
    self.backend.transaction(reverts_iter).await?;
    self.backend.delete_migrations(version, mg).await?;
    Ok(())
  }

  #[allow(
    // Sortable elements are not copyable
    clippy::unnecessary_sort_by
  )]
  /// Applies `rollback` to a set of groups according to the configuration file
  #[inline]
  #[cfg(feature = "std")]
  pub async fn rollback_from_cfg<'a>(
    &'a mut self,
    path: &'a Path,
    versions: &'a [i32],
    files_num: usize,
  ) -> crate::Result<()> {
    let cfg_dir = path.parent().unwrap_or_else(|| Path::new("."));
    let mut dirs_str = parse_cfg(File::open(path)?, cfg_dir)?;
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
  #[cfg(feature = "std")]
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
  #[cfg(feature = "std")]
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

#[cfg(all(feature = "_integration_tests", test))]
pub(crate) mod tests {
  use crate::{Backend, Commands, MigrationGroup};
  use std::path::Path;

  pub(crate) async fn rollback_works<B>(c: &mut Commands<B>)
  where
    B: Backend,
  {
    let path = Path::new("../oapth-test-utils/oapth.cfg");
    c.migrate_from_cfg(path, 128).await.unwrap();
    c.rollback_from_cfg(path, &[0, 0][..], 128).await.unwrap();
    let initial = MigrationGroup::new(1, "initial");
    let initial_migrations = c.backend_mut().migrations(&initial).await.unwrap();
    assert_eq!(initial_migrations.len(), 0);
    let more_stuff = MigrationGroup::new(2, "more_stuff");
    let more_stuff_migrations = c.backend_mut().migrations(&more_stuff).await.unwrap();
    assert_eq!(more_stuff_migrations.len(), 0);
  }
}
