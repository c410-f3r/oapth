use crate::{database::Database, DatabaseTy};
#[cfg(feature = "sm")]
use alloc::{string::String, vec::Vec};

impl Database for () {
  const TY: DatabaseTy = DatabaseTy::Unit;

  type Row = ();

  #[inline]
  async fn execute(&mut self, _: &str) -> crate::Result<()> {
    Ok(())
  }

  #[inline]
  async fn row(&mut self, _: &str) -> crate::Result<Self::Row> {
    Ok(())
  }

  #[inline]
  async fn rows<E>(&mut self, _: &str, _: impl FnMut(Self::Row) -> Result<(), E>) -> Result<(), E>
  where
    E: From<crate::Error>,
  {
    Ok(())
  }

  #[inline]
  async fn transaction(&mut self, _: &str) -> crate::Result<()> {
    Ok(())
  }
}

#[cfg(feature = "sm")]
impl crate::sm::SchemaManagement for () {
  #[inline]
  async fn clear(&mut self, _: (&mut String, &mut Vec<crate::Identifier>)) -> crate::Result<()> {
    Ok(())
  }

  #[inline]
  async fn create_oapth_tables(&mut self) -> crate::Result<()> {
    Ok(())
  }

  #[inline]
  async fn delete_migrations<S>(
    &mut self,
    _: &mut String,
    _: &crate::sm::MigrationGroup<S>,
    _: i32,
  ) -> crate::Result<()>
  where
    S: AsRef<str>,
  {
    Ok(())
  }

  #[inline]
  async fn insert_migrations<'migration, DBS, I, S>(
    &mut self,
    _: &mut String,
    _: &crate::sm::MigrationGroup<S>,
    _: I,
  ) -> crate::Result<()>
  where
    DBS: AsRef<[DatabaseTy]> + 'migration,
    I: Clone + Iterator<Item = &'migration crate::sm::UserMigration<DBS, S>>,
    S: AsRef<str> + 'migration,
  {
    Ok(())
  }

  #[inline]
  async fn migrations<S>(
    &mut self,
    _: &mut String,
    _: &crate::sm::MigrationGroup<S>,
    _: &mut Vec<crate::sm::DbMigration>,
  ) -> crate::Result<()>
  where
    S: AsRef<str>,
  {
    Ok(())
  }

  #[inline]
  async fn table_names(
    &mut self,
    _: &mut String,
    _: &mut Vec<crate::Identifier>,
    _: &str,
  ) -> crate::Result<()> {
    Ok(())
  }
}
