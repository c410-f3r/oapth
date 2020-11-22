use crate::{
  fixed_sql_commands::{
    _delete_migrations, _insert_migrations, _migrations_by_mg_version_query,
    sqlite::{_all_tables, _CREATE_MIGRATION_TABLES},
  },
  BackEnd, BoxFut, Config, DbMigration, Migration, MigrationGroup, _BackEnd,
};
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

impl _BackEnd for Rusqlite {
  #[inline]
  fn all_tables<'a>(&'a mut self, schema: &'a str) -> BoxFut<'a, crate::Result<Vec<String>>> {
    Box::pin(async move {
      let buffer = _all_tables(schema)?;
      Ok(self.query(buffer.as_str(), |r| Ok(r.get::<_, String>(0)?)).await?)
    })
  }

  #[cfg(feature = "dev-tools")]
  #[inline]
  fn clean<'a>(&'a mut self) -> BoxFut<'a, crate::Result<()>> {
    Box::pin(async move { Ok(self.execute(&crate::fixed_sql_commands::sqlite::_clean()?).await?) })
  }

  #[inline]
  fn create_oapth_tables<'a>(&'a mut self) -> BoxFut<'a, crate::Result<()>> {
    self.execute(_CREATE_MIGRATION_TABLES)
  }

  #[inline]
  fn delete_migrations<'a>(
    &'a mut self,
    version: i32,
    mg: &'a MigrationGroup,
  ) -> BoxFut<'a, crate::Result<()>> {
    Box::pin(async move { Ok(_delete_migrations(self, mg, "", version).await?) })
  }

  #[inline]
  fn execute<'a>(&'a mut self, command: &'a str) -> BoxFut<'a, crate::Result<()>> {
    Box::pin(async move { Ok(self.conn.execute_batch(command)?) })
  }

  #[inline]
  fn insert_migrations<'a, I>(
    &'a mut self,
    migrations: I,
    mg: &'a MigrationGroup,
  ) -> BoxFut<'a, crate::Result<()>>
  where
    I: Clone + Iterator<Item = &'a Migration> + 'a,
  {
    Box::pin(_insert_migrations(self, mg, "", migrations))
  }

  #[inline]
  fn migrations<'a>(
    &'a mut self,
    mg: &'a MigrationGroup,
  ) -> BoxFut<'a, crate::Result<Vec<DbMigration>>> {
    Box::pin(async move {
      let fun = |e| {
        if let crate::Error::Rusqlite(inner) = e {
          inner
        } else {
          rusqlite::Error::InvalidQuery
        }
      };
      let buffer = _migrations_by_mg_version_query(mg.version(), "")?;
      Ok(self.query(buffer.as_str(), |row| DbMigration::try_from(row).map_err(fun)).await?)
    })
  }

  #[inline]
  fn transaction<'a, I, S>(&'a mut self, commands: I) -> BoxFut<'a, crate::Result<()>>
  where
    I: Iterator<Item = S> + 'a,
    S: AsRef<str>,
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
