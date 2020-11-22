use crate::{BoxFut, DbMigration, Migration, MigrationGroup};
use alloc::{string::String, vec::Vec};

pub trait _BackEnd {
  fn all_tables<'a>(&'a mut self, schema: &'a str) -> BoxFut<'a, crate::Result<Vec<String>>>;

  #[cfg(feature = "dev-tools")]
  fn clean<'a>(&'a mut self) -> BoxFut<'a, crate::Result<()>>;

  fn create_oapth_tables<'a>(&'a mut self) -> BoxFut<'a, crate::Result<()>>;

  fn delete_migrations<'a>(
    &'a mut self,
    version: i32,
    mg: &'a MigrationGroup,
  ) -> BoxFut<'a, crate::Result<()>>;

  fn execute<'a>(&'a mut self, command: &'a str) -> BoxFut<'a, crate::Result<()>>;

  fn insert_migrations<'a, I>(
    &'a mut self,
    migrations: I,
    mg: &'a MigrationGroup,
  ) -> BoxFut<'a, crate::Result<()>>
  where
    I: Clone + Iterator<Item = &'a Migration> + 'a;

  fn migrations<'a>(
    &'a mut self,
    mg: &'a MigrationGroup,
  ) -> BoxFut<'a, crate::Result<Vec<DbMigration>>>;

  fn transaction<'a, I, S>(&'a mut self, commands: I) -> BoxFut<'a, crate::Result<()>>
  where
    I: Iterator<Item = S> + 'a,
    S: AsRef<str>;
}
