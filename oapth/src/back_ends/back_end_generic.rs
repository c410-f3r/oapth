use crate::{BoxFut, DbMigration, MigrationGroupRef, MigrationRef};
use alloc::{string::String, vec::Vec};
use oapth_commons::Database;

pub trait BackEndGeneric {
  #[oapth_macros::_dev_tools]
  fn clean<'a, 'ret>(&'a mut self) -> BoxFut<'ret, crate::Result<()>>
  where
    'a: 'ret,
    Self: 'ret;

  fn create_oapth_tables<'a, 'ret>(&'a mut self) -> BoxFut<'ret, crate::Result<()>>
  where
    'a: 'ret,
    Self: 'ret;

  fn database() -> Database;

  fn delete_migrations<'a, 'b, 'ret>(
    &'a mut self,
    version: i32,
    mg: MigrationGroupRef<'b>,
  ) -> BoxFut<'ret, crate::Result<()>>
  where
    'a: 'ret,
    'b: 'ret,
    Self: 'ret;

  fn execute<'a, 'b, 'ret>(&'a mut self, command: &'b str) -> BoxFut<'ret, crate::Result<()>>
  where
    'a: 'ret,
    'b: 'ret,
    Self: 'ret;

  fn insert_migrations<'a, 'b, 'c, 'ret, I>(
    &'a mut self,
    migrations: I,
    mg: MigrationGroupRef<'b>,
  ) -> BoxFut<'ret, crate::Result<()>>
  where
    'a: 'ret,
    'b: 'ret,
    'c: 'ret,
    I: Clone + Iterator<Item = MigrationRef<'c, 'c>> + 'ret,
    Self: 'ret;

  fn migrations<'a, 'b, 'ret>(
    &'a mut self,
    mg: MigrationGroupRef<'b>,
  ) -> BoxFut<'ret, crate::Result<Vec<DbMigration>>>
  where
    'a: 'ret,
    'b: 'ret,
    Self: 'ret;

  fn query_string<'a, 'b, 'ret>(
    &'a mut self,
    query: &'b str,
  ) -> BoxFut<'ret, crate::Result<Vec<String>>>
  where
    'a: 'ret,
    'b: 'ret,
    Self: 'ret;

  fn tables<'a, 'b, 'ret>(
    &'a mut self,
    schema: &'b str,
  ) -> BoxFut<'ret, crate::Result<Vec<String>>>
  where
    'a: 'ret,
    'b: 'ret,
    Self: 'ret;

  fn transaction<'a, 'ret, I, S>(&'a mut self, commands: I) -> BoxFut<'ret, crate::Result<()>>
  where
    'a: 'ret,
    I: Iterator<Item = S> + 'ret,
    S: AsRef<str>,
    Self: 'ret;
}
