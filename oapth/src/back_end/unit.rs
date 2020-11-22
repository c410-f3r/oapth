use crate::{BackEnd, BoxFut, DbMigration, Migration, MigrationGroup, _BackEnd};
use alloc::{boxed::Box, string::String, vec::Vec};

impl BackEnd for () {}

impl _BackEnd for () {
  #[inline]
  fn all_tables<'a>(&'a mut self, _: &'a str) -> BoxFut<'a, crate::Result<Vec<String>>> {
    Box::pin(async move { Ok(Vec::new()) })
  }

  #[cfg(feature = "dev-tools")]
  #[inline]
  fn clean<'a>(&'a mut self) -> BoxFut<'a, crate::Result<()>> {
    Box::pin(async move { Ok(()) })
  }

  #[inline]
  fn create_oapth_tables<'a>(&'a mut self) -> BoxFut<'a, crate::Result<()>> {
    Box::pin(async move { Ok(()) })
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
  fn insert_migrations<'a, I>(
    &'a mut self,
    _: I,
    _: &'a MigrationGroup,
  ) -> BoxFut<'a, crate::Result<()>>
  where
    I: Clone + Iterator<Item = &'a Migration> + 'a,
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
  fn transaction<'a, I, S>(&'a mut self, _: I) -> BoxFut<'a, crate::Result<()>>
  where
    I: Iterator<Item = S> + 'a,
    S: AsRef<str>,
  {
    Box::pin(async move { Ok(()) })
  }
}
