use crate::{
  fixed_sql_commands::{
    delete_migrations, insert_migrations, migrations_by_mg_version_query,
    sqlite::{tables, CREATE_MIGRATION_TABLES},
  },
  BackEnd, BackEndGeneric, BoxFut, Config, DbMigration, MigrationRef, MigrationGroupRef,
};
use oapth_commons::Database;
use alloc::string::String;
use core::convert::TryFrom;
use rusqlite::{Connection, Row, NO_PARAMS};

/// Wraps functionalities for the `rusqlite` crate
#[derive(Debug)]
pub struct Rusqlite {
  conn: Connection,
}

impl Rusqlite {
  /// Creates a new instance from all necessary parameters.
  ///
  /// # Example
  ///
  #[cfg_attr(feature = "_integration-tests", doc = "```rust")]
  #[cfg_attr(not(feature = "_integration-tests"), doc = "```ignore,rust")]
  /// #[tokio::main]
  /// # async fn main() -> oapth::Result<()> {
  /// use oapth::{Config, Rusqlite};
  /// let _ = Rusqlite::new(&Config::with_url_from_default_var()?).await?;
  /// # Ok(()) }
  #[inline]
  pub async fn new(config: &Config) -> crate::Result<Self> {
    let real_path = config.url().rsplit("://").next().ok_or(crate::Error::InvalidUrl)?;
    let conn = Connection::open_with_flags(real_path, Default::default())?;
    Ok(Self { conn })
  }

  #[inline]
  async fn query<F, T>(&mut self, query: &str, cb: F) -> crate::Result<Vec<T>>
  where
    F: FnMut(&Row<'_>) -> rusqlite::Result<T>,
  {
    Ok(
      self
        .conn
        .prepare(query)?
        .query_map(NO_PARAMS, cb)?
        .into_iter()
        .collect::<Result<Vec<T>, _>>()?,
    )
  }
}

impl BackEnd for Rusqlite {}

impl BackEndGeneric for Rusqlite {
  #[oapth_macros::_dev_tools]
  #[inline]
  fn clean<'a, 'ret>(&'a mut self) -> BoxFut<'ret, crate::Result<()>>
  where
    'a: 'ret,
    Self: 'ret,
  {
    Box::pin(crate::fixed_sql_commands::sqlite::clean(self))
  }

  #[inline]
  fn create_oapth_tables<'a, 'ret>(&'a mut self) -> BoxFut<'ret, crate::Result<()>>
  where
    'a: 'ret,
    Self: 'ret,
  {
    self.execute(CREATE_MIGRATION_TABLES)
  }

  #[inline]
  fn database() -> Database {
    Database::Sqlite
  }

  #[inline]
  fn delete_migrations<'a, 'b, 'ret>(
    &'a mut self,
    version: i32,
    mg: MigrationGroupRef<'b>,
  ) -> BoxFut<'ret, crate::Result<()>>
  where
    'a: 'ret,
    'b: 'ret,
    Self: 'ret,
  {
    Box::pin(delete_migrations(self, mg, "", version))
  }

  #[inline]
  fn execute<'a, 'b, 'ret>(&'a mut self, command: &'b str) -> BoxFut<'ret, crate::Result<()>>
  where
    'a: 'ret,
    'b: 'ret,
    Self: 'ret,
  {
    Box::pin(async move { Ok(self.conn.execute_batch(command)?) })
  }

  #[inline]
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
    Self: 'ret
  {
    Box::pin(insert_migrations(self, mg, "", migrations))
  }

  #[inline]
  fn migrations<'a, 'b, 'ret>(
    &'a mut self,
    mg: MigrationGroupRef<'b>,
  ) -> BoxFut<'ret, crate::Result<Vec<DbMigration>>>
  where
    'a: 'ret,
    'b: 'ret,
    Self: 'ret,
  {
    Box::pin(async move {
      let fun = |e| {
        if let crate::Error::Rusqlite(inner) = e {
          inner
        } else {
          rusqlite::Error::InvalidQuery
        }
      };
      let buffer = migrations_by_mg_version_query(mg.version(), "")?;
      self.query(buffer.as_str(), |row| DbMigration::try_from(row).map_err(fun)).await
    })
  }

  #[inline]
  fn query_string<'a, 'b, 'ret>(
    &'a mut self,
    query: &'b str,
  ) -> BoxFut<'ret, crate::Result<Vec<String>>>
  where
    'a: 'ret,
    'b: 'ret,
    Self: 'ret,
  {
    Box::pin(self.query(query, |r| r.get::<_, String>(0)))
  }

  #[inline]
  fn tables<'a, 'b, 'ret>(&'a mut self, schema: &'b str) -> BoxFut<'ret, crate::Result<Vec<String>>>
  where
    'a: 'ret,
    'b: 'ret,
    Self: 'ret,
  {
    Box::pin(async move {
      let buffer = tables(schema)?;
      self.query(buffer.as_str(), |r| r.get::<_, String>(0)).await
    })
  }

  #[inline]
  fn transaction<'a, 'ret, I, S>(&'a mut self, commands: I) -> BoxFut<'ret, crate::Result<()>>
  where
    'a: 'ret,
    I: Iterator<Item = S> + 'ret,
    S: AsRef<str>,
    Self: 'ret
  {
    Box::pin(async move {
      let transaction = self.conn.transaction()?;
      for command in commands {
        transaction.execute_batch(command.as_ref())?;
      }
      transaction.commit()?;
      Ok(())
    })
  }
}
