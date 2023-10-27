use crate::{
  sm::{Commands, DbMigration, MigrationGroup, SchemaManagement, UserMigration},
  DatabaseTy,
};
use alloc::{string::String, vec::Vec};
#[cfg(feature = "std")]
use {
  crate::sm::utils::{group_and_migrations_from_path, parse_root_toml},
  std::path::Path,
};

impl<D> Commands<D>
where
  D: SchemaManagement,
{
  /// Rollbacks the migrations of a group to a given `version`.
  ///
  /// Before issuing a rollback, all migrations are validated.
  #[inline]
  pub async fn rollback<'migration, DBS, I, S>(
    &mut self,
    (buffer_cmd, buffer_db_migrations): (&mut String, &mut Vec<DbMigration>),
    mg: &MigrationGroup<S>,
    migrations: I,
    version: i32,
  ) -> crate::Result<()>
  where
    DBS: AsRef<[DatabaseTy]> + 'migration,
    I: Clone + Iterator<Item = &'migration UserMigration<DBS, S>>,
    S: AsRef<str> + 'migration,
  {
    self.database.migrations(buffer_cmd, mg, buffer_db_migrations).await?;
    let filtered_by_db = Self::filter_by_db(migrations);
    Self::do_validate(buffer_db_migrations, filtered_by_db.clone())?;
    for elem in filtered_by_db.map(UserMigration::sql_down) {
      buffer_cmd.push_str(elem.as_ref());
    }
    self.database.transaction(buffer_cmd).await?;
    buffer_cmd.clear();
    self.database.delete_migrations(buffer_cmd, mg, version).await?;
    buffer_db_migrations.clear();
    Ok(())
  }

  /// Applies `rollback` to a set of groups according to the configuration file
  #[inline]
  #[cfg(feature = "std")]
  pub async fn rollback_from_toml(
    &mut self,
    (buffer_cmd, buffer_db_migrations): (&mut String, &mut Vec<DbMigration>),
    path: &Path,
    versions: &[i32],
  ) -> crate::Result<()> {
    let (mut migration_groups, _) = parse_root_toml(path)?;
    if migration_groups.len() != versions.len() {
      return Err(crate::Error::DifferentRollbackVersions);
    }
    migration_groups.sort_by(|a, b| b.cmp(a));
    for (mg, &version) in migration_groups.into_iter().zip(versions) {
      self.do_rollback_from_dir((buffer_cmd, buffer_db_migrations), &mg, version).await?;
    }
    Ok(())
  }

  /// Applies `rollback` to a set of migrations according to a given directory
  #[inline]
  #[cfg(feature = "std")]
  pub async fn rollback_from_dir(
    &mut self,
    buffer: (&mut String, &mut Vec<DbMigration>),
    path: &Path,
    version: i32,
  ) -> crate::Result<()> {
    self.do_rollback_from_dir(buffer, path, version).await
  }

  #[inline]
  #[cfg(feature = "std")]
  async fn do_rollback_from_dir(
    &mut self,
    (buffer_cmd, buffer_db_migrations): (&mut String, &mut Vec<DbMigration>),
    path: &Path,
    version: i32,
  ) -> crate::Result<()> {
    let opt = group_and_migrations_from_path(path, |a, b| b.cmp(a));
    let Ok((mg, mut migrations)) = opt else { return Ok(()) };
    let mut tmp_migrations = Vec::new();
    loop_files!(
      tmp_migrations,
      migrations,
      self.batch_size(),
      self
        .rollback((buffer_cmd, buffer_db_migrations), &mg, tmp_migrations.iter(), version)
        .await?
    );
    Ok(())
  }
}
