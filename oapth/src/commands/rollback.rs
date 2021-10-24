use crate::{BackEnd, Commands, MigrationGroupRef, MigrationRef};
#[oapth_macros::_std]
use {
  crate::{group_and_migrations_from_path, MigrationOwned},
  oapth_commons::parse_root_toml,
  std::path::Path,
};

impl<B> Commands<B>
where
  B: BackEnd,
{
  /// Rollbacks the migrations of a group to a given `version`.
  ///
  /// Before issuing a rollback, all migrations are validated.
  #[inline]
  pub async fn rollback<'a: 'b, 'b, I>(
    &'a mut self,
    mg: MigrationGroupRef<'_>,
    migrations: I,
    version: i32,
  ) -> crate::Result<()>
  where
    I: Clone + DoubleEndedIterator<Item = MigrationRef<'a, 'a>> + 'b,
  {
    let db_migrations = self.back_end.migrations(mg).await?;
    let filtered_by_db = Self::filter_by_db(migrations);
    Self::do_validate(&db_migrations, filtered_by_db.clone())?;
    let reverts_iter = filtered_by_db.map(|el| el.sql_down);
    self.back_end.transaction(reverts_iter).await?;
    self.back_end.delete_migrations(version, mg).await?;
    Ok(())
  }

  /// Applies `rollback` to a set of groups according to the configuration file
  #[inline]
  #[oapth_macros::_std]
  pub async fn rollback_from_cfg(&mut self, path: &Path, versions: &[i32]) -> crate::Result<()> {
    let (mut migration_groups, _) = parse_root_toml(path)?;
    if migration_groups.len() != versions.len() {
      return Err(crate::Error::DifferentRollbackVersions);
    }
    migration_groups.sort_by(|a, b| b.cmp(a));
    let mut buffer = Vec::with_capacity(16);
    for (mg, &version) in migration_groups.into_iter().zip(versions) {
      self.do_rollback_from_dir(&mut buffer, &mg, version).await?;
    }
    Ok(())
  }

  /// Applies `rollback` to a set of migrations according to a given directory
  #[inline]
  #[oapth_macros::_std]
  pub async fn rollback_from_dir(&mut self, path: &Path, version: i32) -> crate::Result<()> {
    let mut buffer = Vec::with_capacity(16);
    self.do_rollback_from_dir(&mut buffer, path, version).await
  }

  #[inline]
  #[oapth_macros::_std]
  async fn do_rollback_from_dir(
    &mut self,
    buffer: &mut Vec<MigrationOwned>,
    path: &Path,
    version: i32,
  ) -> crate::Result<()> {
    let opt = group_and_migrations_from_path(path, |a, b| b.cmp(a));
    let (mg, mut migrations) = if let Ok(rslt) = opt { rslt } else { return Ok(()) };
    loop_files!(
      buffer,
      migrations,
      self.batch_size,
      self.rollback(mg.m_g_ref(), buffer.iter().map(|e| e.m_ref()), version).await?
    );
    Ok(())
  }
}
