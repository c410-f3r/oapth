use crate::{
  binary_seach_migration_by_version, Backend, Commands, DbMigration, Migration, MigrationGroup,
  MigrationParams,
};

impl<B> Commands<B>
where
  B: Backend,
{
  /// Verifies if the provided migrations are a superset of the migrations within the database
  /// by verification their checksums.
  #[inline]
  pub async fn validate<'a, I>(
    &'a mut self,
    mg: &MigrationGroup,
    migrations: I,
  ) -> crate::Result<()>
  where
    I: Iterator<Item = &'a Migration>,
  {
    let db_migrations = self.backend.migrations(mg).await?;
    Self::do_validate(&db_migrations, migrations)
  }

  #[inline]
  pub(crate) fn do_validate<'a, I>(
    db_migrations: &[DbMigration],
    migrations: I,
  ) -> crate::Result<()>
  where
    I: Iterator<Item = &'a Migration>,
  {
    let mut migrations_len: usize = 0;

    for migration in migrations {
      let version = migration.version();
      let opt = binary_seach_migration_by_version(version, &db_migrations);
      let db_migration = if let Some(rslt) = opt {
        rslt.1
      } else {
        continue;
      };

      if migration.common() != db_migration.common() {
        return Err(crate::Error::ValidationDivergentMigrations(version));
      }

      migrations_len = migrations_len.saturating_add(1);
    }

    if migrations_len < db_migrations.len() {
      return Err(crate::Error::ValidationLessMigrationsNum(db_migrations.len(), migrations_len));
    }

    Ok(())
  }
}
