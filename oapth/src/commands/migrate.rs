use crate::{migration_is_divergent, BackEnd, Commands, DbMigration, Migration, MigrationGroup};
#[oapth_macros::std_]
use {
  crate::{group_and_migrations_from_path, parse_root_cfg},
  std::{fs::File, path::Path},
};

impl<B> Commands<B>
where
  B: BackEnd,
{
  /// Migrates everything inside a group that is greater than the last migration version within the
  /// database
  #[inline]
  pub async fn migrate<'a, I>(&'a mut self, mg: &MigrationGroup, migrations: I) -> crate::Result<()>
  where
    I: Clone + Iterator<Item = &'a Migration> + 'a,
  {
    self.back_end.create_oapth_tables().await?;
    let db_migrations = self.back_end.migrations(mg).await?;
    self.do_migrate(&db_migrations, mg, migrations).await
  }

  /// Applies `migrate` to a set of groups according to the configuration file
  #[oapth_macros::std_]
  #[inline]
  pub async fn migrate_from_cfg<'a>(
    &'a mut self,
    path: &'a Path,
    files_num: usize,
  ) -> crate::Result<()> {
    self.back_end.create_oapth_tables().await?;
    let cfg_dir = path.parent().unwrap_or_else(|| Path::new("."));
    let mut buffer = Vec::with_capacity(16);
    let mut dirs_str = parse_root_cfg(File::open(path)?, cfg_dir)?;
    dirs_str.sort();
    for dir_str in dirs_str {
      self.do_migrate_from_dir(&mut buffer, &dir_str, files_num).await?;
    }
    Ok(())
  }

  /// Applies `migrate` to a set of migrations according to a given directory
  #[oapth_macros::std_]
  #[inline]
  pub async fn migrate_from_dir<'a>(
    &'a mut self,
    path: &'a Path,
    files_num: usize,
  ) -> crate::Result<()> {
    self.back_end.create_oapth_tables().await?;
    let mut buffer = Vec::with_capacity(16);
    self.do_migrate_from_dir(&mut buffer, path, files_num).await
  }

  #[inline]
  async fn do_migrate<'a, I>(
    &mut self,
    db_migrations: &[DbMigration],
    mg: &MigrationGroup,
    migrations: I,
  ) -> crate::Result<()>
  where
    B: 'a,
    I: Clone + Iterator<Item = &'a Migration> + 'a,
  {
    let filtered_by_db = Self::filter_by_db(migrations);
    Self::do_validate(&db_migrations, filtered_by_db.clone())?;
    let last_db_mig_version_opt = db_migrations.last().map(|e| e.version());
    let filtered_by_repeatability = filtered_by_db.filter(move |el| {
      if let Some(crate::Repeatability::OnChecksumChange) = el.repeatability() {
        migration_is_divergent(&db_migrations, el)
      } else {
        true
      }
    });
    if let Some(last_db_mig_version) = last_db_mig_version_opt {
      let to_apply = filtered_by_repeatability.filter(move |e| e.version() > last_db_mig_version);
      self.back_end.insert_migrations(to_apply, mg).await?;
    } else {
      self.back_end.insert_migrations(filtered_by_repeatability, mg).await?;
    }
    Ok(())
  }

  #[oapth_macros::std_]
  #[inline]
  async fn do_migrate_from_dir<'a>(
    &'a mut self,
    buffer: &'a mut Vec<Migration>,
    path: &'a Path,
    files_num: usize,
  ) -> crate::Result<()> {
    let opt = group_and_migrations_from_path(path, |a, b| a.cmp(b));
    let (mg, mut migrations) = if let Some(rslt) = opt { rslt } else { return Ok(()) };
    let db_migrations = self.back_end.migrations(&mg).await?;
    loop_files!(
      buffer,
      migrations,
      files_num,
      self.do_migrate(&db_migrations, &mg, buffer.iter()).await?
    );
    Ok(())
  }
}
