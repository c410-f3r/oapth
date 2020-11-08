use crate::{Backend, Commands, Migration, MigrationGroup};

impl<B> Commands<B>
where
  B: Backend,
{
  /// Rollbacks the migrations of a group to a given `version`.
  ///
  /// Before issuing a rollback, all migrations are validated.
  #[inline]
  pub async fn rollback<'a, I>(
    &'a mut self,
    mg: &MigrationGroup,
    migrations: I,
    version: i32,
  ) -> crate::Result<()>
  where
    I: Clone + DoubleEndedIterator<Item = &'a Migration> + 'a,
  {
    let db_migrations = self.backend.migrations(mg).await?;
    Self::do_validate(&db_migrations, migrations.clone())?;
    let reverts_iter = migrations.map(|el| el.sql_down());
    self.backend.transaction(reverts_iter).await?;
    self.backend.delete_migrations(version, mg).await?;
    Ok(())
  }
}
