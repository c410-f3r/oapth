use crate::{is_migration_divergent, Backend, Commands, DbMigration, Migration, MigrationGroup};
use alloc::string::String;
use oapth_commons::{Database, Repeatability};
#[oapth_macros::_std]
use {crate::group_and_migrations_from_path, oapth_commons::parse_root_toml, std::path::Path};

impl<B> Commands<B>
where
  B: Backend,
{
  /// Verifies if the provided migrations are a superset of the migrations within the database
  /// by verification their checksums.
  #[inline]
  pub async fn validate<'migration, DBS, I, S>(
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
    let db_migrations = self.backend.migrations(buffer, mg).await?;
    Self::do_validate(&db_migrations, Self::filter_by_db(migrations))
  }

  /// Applies `validate` to a set of groups according to the configuration file
  #[inline]
  #[oapth_macros::_std]
  pub async fn validate_from_toml(&mut self, path: &Path) -> crate::Result<()> {
    let (mut migration_groups, _) = parse_root_toml(path)?;
    migration_groups.sort();
    for mg in migration_groups {
      self.do_validate_from_dir(&mg).await?;
    }
    Ok(())
  }

  /// Applies `validate` to a set of migrations according to a given directory
  #[inline]
  #[oapth_macros::_std]
  pub async fn validate_from_dir(&mut self, path: &Path) -> crate::Result<()> {
    self.do_validate_from_dir(path).await
  }

  #[inline]
  pub(crate) fn do_validate<'migration, DBS, S, I>(
    db_migrations: &[DbMigration],
    migrations: I,
  ) -> crate::Result<()>
  where
    DBS: AsRef<[Database]> + 'migration,
    I: Iterator<Item = &'migration Migration<DBS, S>>,
    S: AsRef<str> + 'migration,
  {
    let mut migrations_len: usize = 0;
    for migration in migrations {
      match migration.repeatability() {
        Some(Repeatability::Always) => {}
        _ => {
          if is_migration_divergent(db_migrations, migration) {
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
  async fn do_validate_from_dir(&mut self, path: &Path) -> crate::Result<()> {
    let opt = group_and_migrations_from_path(path, |a, b| a.cmp(b));
    let (mg, mut migrations) = if let Ok(rslt) = opt { rslt } else { return Ok(()) };
    let db_migrations = self.backend.migrations(&mut String::new(), &mg).await?;
    let mut tmp_migrations = Vec::new();
    loop_files!(
      tmp_migrations,
      migrations,
      self.batch_size,
      Self::do_validate(&db_migrations, tmp_migrations.iter())?
    );
    Ok(())
  }
}
