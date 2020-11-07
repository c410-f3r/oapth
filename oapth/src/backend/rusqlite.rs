use crate::{
  fixed_sql_commands::{
    _insert_migrations, _migrations_by_group_version_query, _CREATE_MIGRATION_TABLES_SQLITE,
  },
  Backend, BoxFut, Config, DbMigration, Migration, MigrationGroup,
};
use core::convert::TryFrom;
use rusqlite::{params, Connection, NO_PARAMS};

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
  #[cfg_attr(feature = "_integration_tests", doc = "```rust")]
  #[cfg_attr(not(feature = "_integration_tests"), doc = "```ignore,rust")]
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
}

impl Backend for Rusqlite {
  #[inline]
  fn create_oapth_tables<'a>(&'a mut self) -> BoxFut<'a, crate::Result<()>> {
    self.execute(_CREATE_MIGRATION_TABLES_SQLITE)
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
      Ok(
        self
          .conn
          .prepare(_migrations_by_group_version_query(mg.version(), "")?.as_str())?
          .query_map(NO_PARAMS, |row| DbMigration::try_from(row).map_err(fun))?
          .into_iter()
          .collect::<Result<Vec<_>, _>>()?,
      )
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
        transaction.execute(command.as_ref(), params![]).map(usize_to_u64)?;
      }
      transaction.commit()?;
      Ok(())
    })
  }
}

#[allow(
  // Who has a 128bit pointer size computer?
  clippy::as_conversions
)]
#[inline]
fn usize_to_u64(n: usize) -> u64 {
  n as u64
}
