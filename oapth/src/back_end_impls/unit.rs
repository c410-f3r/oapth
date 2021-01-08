use crate::{BackEnd, BackEndGeneric, BoxFut, DbMigration, MigrationGroupRef, MigrationRef};
use alloc::{boxed::Box, string::String, vec::Vec};
use oapth_commons::Database;

impl BackEnd for () {}

impl BackEndGeneric for () {
  #[oapth_macros::_dev_tools]
  #[inline]
  fn clean<'a, 'ret>(&'a mut self) -> BoxFut<'ret, crate::Result<()>>
  where
    'a: 'ret,
    Self: 'ret,
  {
    Box::pin(async move { Ok(()) })
  }

  #[inline]
  fn create_oapth_tables<'a, 'ret>(&'a mut self) -> BoxFut<'ret, crate::Result<()>>
  where
    'a: 'ret,
    Self: 'ret,
  {
    Box::pin(async move { Ok(()) })
  }

  #[inline]
  fn database() -> Database {
    Database::Pg
  }

  #[inline]
  fn delete_migrations<'a, 'b, 'ret>(
    &'a mut self,
    _: i32,
    _: MigrationGroupRef<'b>,
  ) -> BoxFut<'ret, crate::Result<()>>
  where
    'a: 'ret,
    'b: 'ret,
    Self: 'ret,
  {
    Box::pin(async move { Ok(()) })
  }

  #[inline]
  fn execute<'a, 'b, 'ret>(&'a mut self, _: &'b str) -> BoxFut<'ret, crate::Result<()>>
  where
    'a: 'ret,
    'b: 'ret,
    Self: 'ret,
  {
    Box::pin(async move { Ok(()) })
  }

  #[inline]
  fn insert_migrations<'a, 'b, 'c, 'ret, I>(
    &'a mut self,
    _: I,
    _: MigrationGroupRef<'b>,
  ) -> BoxFut<'ret, crate::Result<()>>
  where
    'a: 'ret,
    'b: 'ret,
    'c: 'ret,
    I: Clone + Iterator<Item = MigrationRef<'c, 'c>> + 'ret,
    Self: 'ret,
  {
    Box::pin(async move { Ok(()) })
  }

  #[inline]
  fn migrations<'a, 'b, 'ret>(
    &'a mut self,
    _: MigrationGroupRef<'b>,
  ) -> BoxFut<'ret, crate::Result<Vec<DbMigration>>>
  where
    'a: 'ret,
    'b: 'ret,
    Self: 'ret,
  {
    Box::pin(async move { Ok(Vec::new()) })
  }

  #[inline]
  fn query_string<'a, 'b, 'ret>(
    &'a mut self,
    _: &'b str,
  ) -> BoxFut<'ret, crate::Result<Vec<String>>>
  where
    'a: 'ret,
    'b: 'ret,
    Self: 'ret,
  {
    Box::pin(async move { Ok(Vec::new()) })
  }

  #[inline]
  fn tables<'a, 'b, 'ret>(&'a mut self, _: &'b str) -> BoxFut<'ret, crate::Result<Vec<String>>>
  where
    'a: 'ret,
    'b: 'ret,
    Self: 'ret,
  {
    Box::pin(async move { Ok(Vec::new()) })
  }

  #[inline]
  fn transaction<'a, 'ret, I, S>(&'a mut self, _: I) -> BoxFut<'ret, crate::Result<()>>
  where
    'a: 'ret,
    I: Iterator<Item = S> + 'ret,
    Self: 'ret,
  {
    Box::pin(async move { Ok(()) })
  }
}
