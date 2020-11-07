use crate::{Backend, Commands, Migration, MigrationGroup};
use arrayvec::ArrayString;
use core::fmt::Write;

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
    let mut buffer = ArrayString::<[u8; 128]>::new();
    buffer.write_fmt(format_args!(
      "DELETE FROM migrations WHERE _oapth_migration_group_version = {} && mg > {}",
      mg.version(),
      version
    ))?;
    let delete_sql = [buffer];
    let delete_sql_iter = delete_sql.iter().map(|el| el.as_str());
    self.backend.transaction(reverts_iter.chain(delete_sql_iter)).await?;
    Ok(())
  }
}
