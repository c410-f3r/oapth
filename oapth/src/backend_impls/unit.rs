use crate::{Backend, BackendGeneric, DbMigration, Migration, MigrationGroup};
use alloc::{boxed::Box, string::String, vec::Vec};
use oapth_commons::Database;

impl Backend for () {}

#[async_trait::async_trait]
impl BackendGeneric for () {
  #[oapth_macros::_dev_tools]
  #[inline]
  async fn clean(&mut self, _: &mut String) -> crate::Result<()> {
    Ok(())
  }

  #[inline]
  async fn create_oapth_tables(&mut self) -> crate::Result<()> {
    Ok(())
  }

  #[inline]
  fn database() -> Database {
    Database::Unit
  }

  #[inline]
  async fn delete_migrations<S>(
    &mut self,
    _: &mut String,
    _: i32,
    _: &MigrationGroup<S>,
  ) -> crate::Result<()>
  where
    S: AsRef<str> + Send + Sync,
  {
    Ok(())
  }

  #[inline]
  async fn execute(&mut self, _: &str) -> crate::Result<()> {
    Ok(())
  }

  #[inline]
  async fn insert_migrations<'migration, DBS, I, S>(
    &mut self,
    _: &mut String,
    _: I,
    _: &MigrationGroup<S>,
  ) -> crate::Result<()>
  where
    DBS: AsRef<[Database]> + 'migration,
    I: Clone + Iterator<Item = &'migration Migration<DBS, S>> + Send + Sync,
    S: AsRef<str> + Send + Sync + 'migration,
  {
    Ok(())
  }

  #[inline]
  async fn migrations<S>(
    &mut self,
    _: &mut String,
    _: &MigrationGroup<S>,
  ) -> crate::Result<Vec<DbMigration>>
  where
    S: AsRef<str> + Send + Sync,
  {
    Ok(Vec::new())
  }

  #[inline]
  async fn query_string(&mut self, _: &str) -> crate::Result<Vec<String>> {
    Ok(Vec::new())
  }

  #[inline]
  async fn tables(&mut self, _: &str) -> crate::Result<Vec<String>> {
    Ok(Vec::new())
  }

  #[inline]
  async fn transaction<I, S>(&mut self, _: I) -> crate::Result<()>
  where
    I: Iterator<Item = S> + Send,
    S: AsRef<str> + Send + Sync,
  {
    Ok(())
  }
}
