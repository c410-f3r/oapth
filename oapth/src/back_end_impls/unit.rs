use crate::{BackEnd, BackEndGeneric, BoxFut, Database, DbMigration, Migration, MigrationGroup};
use alloc::{boxed::Box, string::String, vec::Vec};

impl BackEnd for () {}

impl BackEndGeneric for () {
  #[oapth_macros::dev_tools_]
  #[inline]
  fn clean<'a>(&'a mut self) -> BoxFut<'a, crate::Result<()>> {
    Box::pin(async move { Ok(()) })
  }

  #[inline]
  fn create_oapth_tables<'a>(&'a mut self) -> BoxFut<'a, crate::Result<()>> {
    Box::pin(async move { Ok(()) })
  }

  #[inline]
  fn database() -> Database {
    Database::Pg
  }

  #[inline]
  fn execute<'a>(&'a mut self, _: &'a str) -> BoxFut<'a, crate::Result<()>> {
    Box::pin(async move { Ok(()) })
  }

  #[inline]
  fn delete_migrations<'a>(
    &'a mut self,
    _: i32,
    _: &'a MigrationGroup,
  ) -> BoxFut<'a, crate::Result<()>> {
    Box::pin(async move { Ok(()) })
  }

  #[inline]
  fn insert_migrations<'a, 'b, 'c, 'ret, I>(
    &'a mut self,
    _: I,
    _: &'b MigrationGroup,
  ) -> BoxFut<'ret, crate::Result<()>>
  where
    'a: 'ret,
    'b: 'ret,
    'c: 'ret,
    I: Clone + Iterator<Item = &'c Migration> + 'ret,
    Self: 'ret,
  {
    Box::pin(async move { Ok(()) })
  }

  #[inline]
  fn migrations<'a>(
    &'a mut self,
    _: &'a MigrationGroup,
  ) -> BoxFut<'a, crate::Result<Vec<DbMigration>>> {
    Box::pin(async move { Ok(Vec::new()) })
  }

  #[inline]
  fn query_string<'a>(&'a mut self, _: &'a str) -> BoxFut<'a, crate::Result<Vec<String>>> {
    Box::pin(async move { Ok(Vec::new()) })
  }

  #[inline]
  fn tables<'a>(&'a mut self, _: &'a str) -> BoxFut<'a, crate::Result<Vec<String>>> {
    Box::pin(async move { Ok(Vec::new()) })
  }

  #[inline]
  fn transaction<'a, I, S>(&'a mut self, _: I) -> BoxFut<'a, crate::Result<()>>
  where
    I: Iterator<Item = S> + 'a,
    S: AsRef<str>,
  {
    Box::pin(async move { Ok(()) })
  }
}
