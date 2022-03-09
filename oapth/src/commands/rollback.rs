use alloc::string::String;
use oapth_commons::Database;

use crate::{Backend, Commands, Migration, MigrationGroup};
#[oapth_macros::_std]
use {crate::group_and_migrations_from_path, oapth_commons::parse_root_toml, std::path::Path};

impl<B> Commands<B>
where
  B: Backend,
{
  /// Rollbacks the migrations of a group to a given `version`.
  ///
  /// Before issuing a rollback, all migrations are validated.
  #[inline]
  pub async fn rollback<'migration, DBS, I, S>(
    &mut self,
    buffer: &mut String,
    mg: &MigrationGroup<S>,
    migrations: I,
    version: i32,
  ) -> crate::Result<()>
  where
    DBS: AsRef<[Database]> + 'migration,
    I: Clone + Iterator<Item = &'migration Migration<DBS, S>> + Send + Sync,
    S: AsRef<str> + Send + Sync + 'migration,
  {
    let db_migrations = self.backend.migrations(buffer, mg).await?;
    let filtered_by_db = Self::filter_by_db(migrations);
    Self::do_validate(&db_migrations, filtered_by_db.clone())?;
    let reverts_iter = filtered_by_db.map(|el| &el.sql_down);
    self.backend.transaction(reverts_iter).await?;
    self.backend.delete_migrations(buffer, version, mg).await?;
    Ok(())
  }

  /// Applies `rollback` to a set of groups according to the configuration file
  #[inline]
  #[oapth_macros::_std]
  pub async fn rollback_from_toml(
    &mut self,
    buffer: &mut String,
    path: &Path,
    versions: &[i32],
  ) -> crate::Result<()> {
    let (mut migration_groups, _) = parse_root_toml(path)?;
    if migration_groups.len() != versions.len() {
      return Err(crate::Error::DifferentRollbackVersions);
    }
    migration_groups.sort_by(|a, b| b.cmp(a));
    for (mg, &version) in migration_groups.into_iter().zip(versions) {
      self.do_rollback_from_dir(buffer, &mg, version).await?;
    }
    Ok(())
  }

  /// Applies `rollback` to a set of migrations according to a given directory
  #[inline]
  #[oapth_macros::_std]
  pub async fn rollback_from_dir(
    &mut self,
    buffer: &mut String,
    path: &Path,
    version: i32,
  ) -> crate::Result<()> {
    self.do_rollback_from_dir(buffer, path, version).await
  }

  #[inline]
  #[oapth_macros::_std]
  async fn do_rollback_from_dir(
    &mut self,
    buffer: &mut String,
    path: &Path,
    version: i32,
  ) -> crate::Result<()> {
    let opt = group_and_migrations_from_path(path, |a, b| b.cmp(a));
    let (mg, mut migrations) = if let Ok(rslt) = opt { rslt } else { return Ok(()) };
    let mut tmp_migrations = Vec::new();
    loop_files!(
      tmp_migrations,
      migrations,
      self.batch_size,
      self.rollback(buffer, &mg, tmp_migrations.iter(), version).await?
    );
    Ok(())
  }
}
