use crate::{DbMigration, Migration, MigrationGroup};
use alloc::{boxed::Box, string::String, vec::Vec};
use oapth_commons::Database;

#[async_trait::async_trait]
pub trait BackendGeneric {
  #[oapth_macros::_dev_tools]
  async fn clean(&mut self, buffer: &mut String) -> crate::Result<()>;

  async fn create_oapth_tables(&mut self) -> crate::Result<()>;

  fn database() -> Database;

  async fn delete_migrations<S>(
    &mut self,
    buffer: &mut String,
    version: i32,
    mg: &MigrationGroup<S>,
  ) -> crate::Result<()>
  where
    S: AsRef<str> + Send + Sync;

  async fn execute(&mut self, command: &str) -> crate::Result<()>;

  async fn insert_migrations<'migration, DBS, I, S>(
    &mut self,
    buffer: &mut String,
    migrations: I,
    mg: &MigrationGroup<S>,
  ) -> crate::Result<()>
  where
    DBS: AsRef<[Database]> + 'migration,
    I: Clone + Iterator<Item = &'migration Migration<DBS, S>> + Send + Sync,
    S: AsRef<str> + Send + Sync + 'migration;

  async fn migrations<S>(
    &mut self,
    buffer: &mut String,
    mg: &MigrationGroup<S>,
  ) -> crate::Result<Vec<DbMigration>>
  where
    S: AsRef<str> + Send + Sync;

  async fn query_string(&mut self, query: &str) -> crate::Result<Vec<String>>;

  async fn tables(&mut self, schema: &str) -> crate::Result<Vec<String>>;

  async fn transaction<I, S>(&mut self, commands: I) -> crate::Result<()>
  where
    I: Iterator<Item = S> + Send,
    S: AsRef<str> + Send + Sync;
}
