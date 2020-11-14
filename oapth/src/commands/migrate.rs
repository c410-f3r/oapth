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
  /// Migrates everything inside a group that is greater than the last migration version within the
  /// database
  #[inline]
  pub async fn migrate<'a, I>(
    &'a mut self,
    mg: &'a MigrationGroup,
    migrations: I,
  ) -> crate::Result<()>
  where
    I: Clone + Iterator<Item = &'a Migration> + 'a,
  {
    self.backend.create_oapth_tables().await?;
    self.do_migrate(mg, migrations).await
  }

  /// Applies `migrate` to a set of groups according to the configuration file
  #[cfg(feature = "std")]
  #[inline]
  pub async fn migrate_from_cfg<'a>(
    &'a mut self,
    path: &'a Path,
    files_num: usize,
  ) -> crate::Result<()> {
    self.backend.create_oapth_tables().await?;
    let cfg_dir = path.parent().unwrap_or_else(|| Path::new("."));
    let mut buffer = Vec::with_capacity(16);
    let mut dirs_str = parse_cfg(File::open(path)?, cfg_dir)?;
    dirs_str.sort();
    for dir_str in dirs_str {
      self.do_migrate_from_dir(&mut buffer, &dir_str, files_num).await?;
    }
    Ok(())
  }

  /// Applies `migrate` to a set of migrations according to a given directory
  #[cfg(feature = "std")]
  #[inline]
  pub async fn migrate_from_dir<'a>(
    &'a mut self,
    path: &'a Path,
    files_num: usize,
  ) -> crate::Result<()> {
    self.backend.create_oapth_tables().await?;
    let mut buffer = Vec::with_capacity(16);
    self.do_migrate_from_dir(&mut buffer, path, files_num).await
  }

  #[inline]
  async fn do_migrate<'a, I>(
    &'a mut self,
    mg: &'a MigrationGroup,
    migrations: I,
  ) -> crate::Result<()>
  where
    I: Clone + Iterator<Item = &'a Migration> + 'a,
  {
    let db_migrations = self.backend.migrations(mg).await?;
    self.do_validate(&db_migrations, migrations.clone())?;
    if let Some(rslt) = db_migrations.last() {
      let last_db_mig_version = rslt.version();
      let to_apply = migrations.filter(move |el| el.version() > last_db_mig_version);
      self.backend.insert_migrations(to_apply, mg).await?;
    } else {
      self.backend.insert_migrations(migrations, mg).await?;
    }
    Ok(())
  }

  #[cfg(feature = "std")]
  #[inline]
  async fn do_migrate_from_dir<'a>(
    &'a mut self,
    buffer: &'a mut Vec<Migration>,
    path: &'a Path,
    files_num: usize,
  ) -> crate::Result<()> {
    let opt = group_and_migrations_from_path(path, |a, b| a.cmp(b));
    let (mg, mut migrations) = if let Some(rslt) = opt { rslt } else { return Ok(()) };
    loop_files!(buffer, migrations, files_num, self.do_migrate(&mg, buffer.iter()).await?);
    Ok(())
  }
}

#[cfg(all(feature = "_integration_tests", test))]
pub(crate) mod tests {
  use crate::{Backend, Commands, MigrationGroup};
  use std::path::Path;

  pub(crate) async fn migrate_works<B>(c: &mut Commands<B>)
  where
    B: Backend,
  {
    let path = Path::new("../oapth-test-utils/oapth.cfg");
    c.migrate_from_cfg(path, 128).await.unwrap();
    let initial = MigrationGroup::new(1, "initial");
    let initial_migrations = c.backend_mut().migrations(&initial).await.unwrap();
    assert_eq!(initial_migrations.len(), 4);
    assert_eq!(initial_migrations[0].checksum(), "11315267835087000498");
    assert_eq!(initial_migrations[0].version(), 1);
    assert_eq!(initial_migrations[0].name(), "create_author");
    let more_stuff = MigrationGroup::new(2, "more_stuff");
    let more_stuff_migrations = c.backend_mut().migrations(&more_stuff).await.unwrap();
    assert_eq!(more_stuff_migrations.len(), 1);
    assert_eq!(more_stuff_migrations[0].checksum(), "4849485378697205622");
    assert_eq!(more_stuff_migrations[0].version(), 1);
    assert_eq!(more_stuff_migrations[0].name(), "create_apple");
    c.rollback_from_cfg(path, &[0, 0][..], 128).await.unwrap();
  }
}
