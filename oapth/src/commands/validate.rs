use crate::{
  binary_seach_migration_by_version, BackEnd, Commands, DbMigration, Migration, MigrationGroup,
};
#[oapth_macros::std_]
use {
  crate::{group_and_migrations_from_path, parse_cfg},
  std::{fs::File, path::Path},
};

impl<B> Commands<B>
where
  B: BackEnd,
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
    I: Clone + Iterator<Item = &'a Migration>,
  {
    let db_migrations = self.back_end.migrations(mg).await?;
    self.do_validate(&db_migrations, Self::filter_by_db(migrations))
  }

  /// Applies `validate` to a set of groups according to the configuration file
  #[inline]
  #[oapth_macros::std_]
  pub async fn validate_from_cfg<'a>(
    &'a mut self,
    path: &'a Path,
    files_num: usize,
  ) -> crate::Result<()> {
    let cfg_dir = path.parent().unwrap_or_else(|| Path::new("."));
    let mut buffer = Vec::with_capacity(16);
    let mut dirs_str = parse_cfg(File::open(path)?, cfg_dir)?;
    dirs_str.sort();
    for dir_str in dirs_str {
      self.do_validate_from_dir(&mut buffer, &dir_str, files_num).await?;
    }
    Ok(())
  }

  /// Applies `validate` to a set of migrations according to a given directory
  #[inline]
  #[oapth_macros::std_]
  pub async fn validate_from_dir<'a>(
    &'a mut self,
    path: &'a Path,
    files_num: usize,
  ) -> crate::Result<()> {
    let mut buffer = Vec::with_capacity(16);
    self.do_validate_from_dir(&mut buffer, path, files_num).await
  }

  #[inline]
  pub(crate) fn do_validate<'a, I>(
    &mut self,
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
        rslt
      } else {
        continue;
      };

      if migration.checksum() != db_migration.checksum()
        || migration.name() != db_migration.name()
        || migration.version() != db_migration.version()
      {
        return Err(crate::Error::ValidationDivergentMigrations(version));
      }

      migrations_len = migrations_len.saturating_add(1);
    }

    if migrations_len < db_migrations.len() {
      return Err(crate::Error::ValidationLessMigrationsNum(db_migrations.len(), migrations_len));
    }

    Ok(())
  }

  #[inline]
  #[oapth_macros::std_]
  async fn do_validate_from_dir<'a>(
    &'a mut self,
    buffer: &'a mut Vec<Migration>,
    path: &'a Path,
    files_num: usize,
  ) -> crate::Result<()> {
    let opt = group_and_migrations_from_path(path, |a, b| a.cmp(b));
    let (mg, mut migrations) = if let Some(rslt) = opt { rslt } else { return Ok(()) };
    let db_migrations = self.back_end.migrations(&mg).await?;
    loop_files!(buffer, migrations, files_num, self.do_validate(&db_migrations, buffer.iter())?);
    Ok(())
  }
}
