use crate::{Backend, Commands, Migration, MigrationGroup};

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

  #[inline]
  pub(crate) async fn do_migrate<'a, I>(
    &'a mut self,
    mg: &'a MigrationGroup,
    migrations: I,
  ) -> crate::Result<()>
  where
    I: Clone + Iterator<Item = &'a Migration> + 'a,
  {
    let db_migrations = self.backend.migrations(mg).await?;
    Self::do_validate(&db_migrations, migrations.clone())?;
    if let Some(rslt) = db_migrations.last() {
      let last_db_mig_version = rslt.common.version;
      let to_apply = migrations.filter(move |el| el.version() > last_db_mig_version);
      self.backend.insert_migrations(to_apply, mg).await?;
    } else {
      self.backend.insert_migrations(migrations, mg).await?;
    }
    Ok(())
  }
}
