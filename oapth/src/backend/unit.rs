use crate::{
  Backend, BoxFut, DbMigration, Migration,
  MigrationGroup,
};
use alloc::vec::Vec;

impl Backend for () {
  #[inline]
  fn create_oapth_tables<'a>(&'a mut self) -> BoxFut<'a, crate::Result<()>> {
    Box::pin(async move { Ok(()) })
  }

  #[inline]
  fn execute<'a>(&'a mut self, _: &'a str) -> BoxFut<'a, crate::Result<()>> {
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
