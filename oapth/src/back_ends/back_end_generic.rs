use crate::{BoxFut, Database, DbMigration, Migration, MigrationGroup};
use alloc::{string::String, vec::Vec};

pub trait BackEndGeneric {
  #[oapth_macros::dev_tools_]
  fn clean<'a>(&'a mut self) -> BoxFut<'a, crate::Result<()>>;

  fn create_oapth_tables<'a>(&'a mut self) -> BoxFut<'a, crate::Result<()>>;

  fn delete_migrations<'a>(
    &'a mut self,
    version: i32,
    mg: &'a MigrationGroup,
  ) -> BoxFut<'a, crate::Result<()>>;

  fn database() -> Database;

  fn execute<'a>(&'a mut self, command: &'a str) -> BoxFut<'a, crate::Result<()>>;

  fn insert_migrations<'a, 'b, 'c, 'ret, I>(
    &'a mut self,
    migrations: I,
    mg: &'b MigrationGroup,
  ) -> BoxFut<'ret, crate::Result<()>>
  where
    'a: 'ret,
    'b: 'ret,
    'c: 'ret,
    I: Clone + Iterator<Item = &'c Migration> + 'ret,
    Self: 'ret;

  fn migrations<'a>(
    &'a mut self,
    mg: &'a MigrationGroup,
  ) -> BoxFut<'a, crate::Result<Vec<DbMigration>>>;

  fn query_string<'a>(&'a mut self, query: &'a str) -> BoxFut<'a, crate::Result<Vec<String>>>;

  fn tables<'a>(&'a mut self, schema: &'a str) -> BoxFut<'a, crate::Result<Vec<String>>>;

  fn transaction<'a, I, S>(&'a mut self, commands: I) -> BoxFut<'a, crate::Result<()>>
  where
    I: Iterator<Item = S> + 'a,
    S: AsRef<str>;
}
