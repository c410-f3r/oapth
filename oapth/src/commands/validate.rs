use crate::{
  is_migration_divergent, BackEnd, Commands, DbMigration, MigrationGroupRef, MigrationRef,
};
use oapth_commons::Repeatability;
#[oapth_macros::_std]
use {
  crate::{group_and_migrations_from_path, MigrationOwned},
  oapth_commons::parse_root_cfg,
  std::path::Path,
};

impl<B> Commands<B>
where
  B: BackEnd,
{
  /// Verifies if the provided migrations are a superset of the migrations within the database
  /// by verification their checksums.
  #[inline]
  pub async fn validate<'a, 'b, I>(
    &mut self,
    mg: MigrationGroupRef<'_>,
    migrations: I,
  ) -> crate::Result<()>
  where
    I: Clone + Iterator<Item = MigrationRef<'a, 'a>> + 'b,
  {
    let db_migrations = self.back_end.migrations(mg).await?;
    Self::do_validate(&db_migrations, Self::filter_by_db(migrations))
  }

  /// Applies `validate` to a set of groups according to the configuration file
  #[inline]
  #[oapth_macros::_std]
  pub async fn validate_from_cfg(&mut self, path: &Path) -> crate::Result<()> {
    let mut buffer = Vec::with_capacity(16);
    let mut dirs_str = parse_root_cfg(path)?;
    dirs_str.sort();
    for dir_str in dirs_str {
      self.do_validate_from_dir(&mut buffer, &dir_str).await?;
    }
    Ok(())
  }

  /// Applies `validate` to a set of migrations according to a given directory
  #[inline]
  #[oapth_macros::_std]
  pub async fn validate_from_dir(&mut self, path: &Path) -> crate::Result<()> {
    let mut buffer = Vec::with_capacity(16);
    self.do_validate_from_dir(&mut buffer, path).await
  }

  #[inline]
  pub(crate) fn do_validate<'a, 'b, I>(
    db_migrations: &[DbMigration],
    migrations: I,
  ) -> crate::Result<()>
  where
    I: Iterator<Item = MigrationRef<'a, 'a>> + 'b,
  {
    let mut migrations_len: usize = 0;
    for migration in migrations {
      match migration.repeatability() {
        Some(Repeatability::Always) => {}
        _ => {
          if is_migration_divergent(db_migrations, &migration) {
            return Err(crate::Error::ValidationDivergentMigrations(migration.version()));
          }
        }
      }
      migrations_len = migrations_len.saturating_add(1);
    }
    if migrations_len < db_migrations.len() {
      return Err(crate::Error::ValidationLessMigrationsNum(db_migrations.len(), migrations_len));
    }
    Ok(())
  }

  #[inline]
  #[oapth_macros::_std]
  async fn do_validate_from_dir(
    &mut self,
    buffer: &mut Vec<MigrationOwned>,
    path: &Path,
  ) -> crate::Result<()> {
    let opt = group_and_migrations_from_path(path, |a, b| a.cmp(b));
    let (mg, mut migrations) = if let Ok(rslt) = opt { rslt } else { return Ok(()) };
    let db_migrations = self.back_end.migrations(mg.m_g_ref()).await?;
    loop_files!(
      buffer,
      migrations,
      self.batch_size,
      Self::do_validate(&db_migrations, buffer.iter().map(|e| e.m_ref()))?
    );
    Ok(())
  }
}
