use crate::{BackEnd, Commands, DbMigration, MigrationGroupRef, MigrationRef};
#[oapth_macros::_std]
use {
  crate::{group_and_migrations_from_path, MigrationOwned},
  arrayvec::ArrayVec,
  oapth_commons::parse_root_toml,
  std::path::{Path, PathBuf},
};

impl<B> Commands<B>
where
  B: BackEnd,
{
  /// Migrates everything inside a group that is greater than the last migration version within the
  /// database
  #[inline]
  pub async fn migrate<'a, 'b, I>(
    &mut self,
    mg: MigrationGroupRef<'_>,
    migrations: I,
  ) -> crate::Result<()>
  where
    I: Clone + Iterator<Item = MigrationRef<'a, 'a>> + 'b,
  {
    self.back_end.create_oapth_tables().await?;
    let db_migrations = self.back_end.migrations(mg).await?;
    self.do_migrate(&db_migrations, mg, migrations).await
  }

  /// Applies `migrate` to a set of migration groups according to the configuration file.
  #[oapth_macros::_std]
  #[inline]
  pub async fn migrate_from_cfg_path(&mut self, path: &Path) -> crate::Result<()> {
    let (migration_groups, _) = parse_root_toml(path)?;
    self.migrate_from_groups_paths(migration_groups).await?;
    Ok(())
  }

  /// Applies `migrate` to a set of migrations according to a given directory
  #[oapth_macros::_std]
  #[inline]
  pub async fn migrate_from_dir(&mut self, path: &Path) -> crate::Result<()> {
    self.back_end.create_oapth_tables().await?;
    let mut buffer = Vec::with_capacity(16);
    self.do_migrate_from_dir(&mut buffer, path).await
  }

  /// Applies `migrate` to a set of migrations according to a given set of groups
  #[inline]
  pub async fn migrate_from_groups<'a, 'b, G, M>(&mut self, groups: G) -> crate::Result<()>
  where
    G: Iterator<Item = (MigrationGroupRef<'a>, M)> + 'b,
    M: Clone + Iterator<Item = MigrationRef<'a, 'a>> + 'b,
  {
    self.back_end.create_oapth_tables().await?;
    for (mg, m) in groups {
      let db_migrations = self.back_end.migrations(mg).await?;
      self.do_migrate(&db_migrations, mg, m).await?;
    }
    Ok(())
  }

  /// Applies `migrate` to the set of provided migration groups paths.
  #[oapth_macros::_std]
  #[inline]
  pub async fn migrate_from_groups_paths(
    &mut self,
    mut migration_groups: ArrayVec<PathBuf, 8>,
  ) -> crate::Result<()> {
    self.back_end.create_oapth_tables().await?;
    let mut buffer = Vec::with_capacity(16);
    migration_groups.sort();
    for mg in migration_groups {
      self.do_migrate_from_dir(&mut buffer, &mg).await?;
    }
    Ok(())
  }

  #[inline]
  async fn do_migrate<'a, 'b, I>(
    &mut self,
    db_migrations: &[DbMigration],
    mg: MigrationGroupRef<'_>,
    migrations: I,
  ) -> crate::Result<()>
  where
    I: Clone + Iterator<Item = MigrationRef<'a, 'a>> + 'b,
  {
    let filtered_by_db = Self::filter_by_db(migrations);
    Self::do_validate(db_migrations, filtered_by_db.clone())?;
    let last_db_mig_version_opt = db_migrations.last().map(|e| e.version());
    if let Some(last_db_mig_version) = last_db_mig_version_opt {
      let to_apply = filtered_by_db.filter(move |e| e.version() > last_db_mig_version);
      self.back_end.insert_migrations(to_apply, mg).await?;
    } else {
      self.back_end.insert_migrations(filtered_by_db, mg).await?;
    }
    Ok(())
  }

  #[oapth_macros::_std]
  #[inline]
  async fn do_migrate_from_dir(
    &mut self,
    buffer: &mut Vec<MigrationOwned>,
    path: &Path,
  ) -> crate::Result<()> {
    let (mg, mut migrations) = group_and_migrations_from_path(path, |a, b| a.cmp(b))?;
    let db_migrations = self.back_end.migrations(mg.m_g_ref()).await?;
    loop_files!(
      buffer,
      migrations,
      self.batch_size,
      self.do_migrate(&db_migrations, mg.m_g_ref(), buffer.iter().map(|e| e.m_ref())).await?
    );
    Ok(())
  }
}
