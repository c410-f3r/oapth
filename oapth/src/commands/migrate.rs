use crate::{Backend, Commands, DbMigration, Migration, MigrationGroup};
use alloc::string::String;
use oapth_commons::Database;
#[oapth_macros::_std]
use {
  crate::group_and_migrations_from_path,
  arrayvec::ArrayVec,
  oapth_commons::parse_root_toml,
  std::path::{Path, PathBuf},
};

type MigrationFromGroups<'slice, 'migration_group, 'migration_slice, DBS, S> =
  &'slice [(&'migration_group MigrationGroup<S>, &'migration_slice [Migration<DBS, S>])];

impl<B> Commands<B>
where
  B: Backend,
{
  /// Migrates everything inside a group that is greater than the last migration version within the
  /// database
  #[inline]
  pub async fn migrate<'migration, DBS, I, S>(
    &mut self,
    buffer: &mut String,
    mg: &MigrationGroup<S>,
    migrations: I,
  ) -> crate::Result<()>
  where
    DBS: AsRef<[Database]> + 'migration,
    I: Clone + Iterator<Item = &'migration Migration<DBS, S>> + Send + Sync,
    S: AsRef<str> + Send + Sync + 'migration,
  {
    self.backend.create_oapth_tables().await?;
    let db_migrations = self.backend.migrations(buffer, mg).await?;
    self.do_migrate(buffer, &db_migrations, mg, migrations).await
  }

  /// Applies `migrate` to a set of migration groups according to the configuration file.
  #[oapth_macros::_std]
  #[inline]
  pub async fn migrate_from_toml_path(
    &mut self,
    buffer: &mut String,
    path: &Path,
  ) -> crate::Result<()> {
    let (migration_groups, _) = parse_root_toml(path)?;
    self.migrate_from_groups_paths(buffer, migration_groups).await?;
    Ok(())
  }

  /// Applies `migrate` to a set of migrations according to a given directory
  #[oapth_macros::_std]
  #[inline]
  pub async fn migrate_from_dir(&mut self, buffer: &mut String, path: &Path) -> crate::Result<()> {
    self.backend.create_oapth_tables().await?;
    self.do_migrate_from_dir(buffer, path).await
  }

  /// Applies `migrate` to a set of migrations according to a given set of groups
  #[inline]
  pub async fn migrate_from_groups<DBS, S>(
    &mut self,
    buffer: &mut String,
    groups: MigrationFromGroups<'_, '_, '_, DBS, S>,
  ) -> crate::Result<()>
  where
    DBS: AsRef<[Database]> + Send + Sync,
    S: AsRef<str> + Send + Sync,
  {
    self.backend.create_oapth_tables().await?;
    for (mg, m) in groups.iter() {
      let db_migrations = self.backend.migrations(buffer, mg).await?;
      self.do_migrate(buffer, &db_migrations, mg, m.iter()).await?;
    }
    Ok(())
  }

  /// Applies `migrate` to the set of provided migration groups paths.
  #[oapth_macros::_std]
  #[inline]
  pub async fn migrate_from_groups_paths(
    &mut self,
    buffer: &mut String,
    mut migration_groups: ArrayVec<PathBuf, 8>,
  ) -> crate::Result<()> {
    self.backend.create_oapth_tables().await?;
    migration_groups.sort();
    for mg in migration_groups {
      self.do_migrate_from_dir(buffer, &mg).await?;
    }
    Ok(())
  }

  #[inline]
  async fn do_migrate<'migration, DBS, I, S>(
    &mut self,
    buffer: &mut String,
    db_migrations: &[DbMigration],
    mg: &MigrationGroup<S>,
    migrations: I,
  ) -> crate::Result<()>
  where
    DBS: AsRef<[Database]> + 'migration,
    I: Clone + Iterator<Item = &'migration Migration<DBS, S>> + Send + Sync,
    S: AsRef<str> + Send + Sync + 'migration,
  {
    let filtered_by_db = Self::filter_by_db(migrations);
    Self::do_validate(db_migrations, filtered_by_db.clone())?;
    let last_db_mig_version_opt = db_migrations.last().map(|e| e.version());
    if let Some(last_db_mig_version) = last_db_mig_version_opt {
      let to_apply = filtered_by_db.filter(move |e| e.version() > last_db_mig_version);
      self.backend.insert_migrations(buffer, to_apply, mg).await?;
    } else {
      self.backend.insert_migrations(buffer, filtered_by_db, mg).await?;
    }
    Ok(())
  }

  #[oapth_macros::_std]
  #[inline]
  async fn do_migrate_from_dir(&mut self, buffer: &mut String, path: &Path) -> crate::Result<()> {
    let (mg, mut migrations) = group_and_migrations_from_path(path, |a, b| a.cmp(b))?;
    let db_migrations = self.backend.migrations(buffer, &mg).await?;
    let mut tmp_migrations = Vec::new();
    loop_files!(
      tmp_migrations,
      migrations,
      self.batch_size,
      self.do_migrate(buffer, &db_migrations, &mg, tmp_migrations.iter()).await?
    );
    Ok(())
  }
}
