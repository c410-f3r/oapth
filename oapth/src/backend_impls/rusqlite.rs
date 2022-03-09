use crate::{
  fixed_sql_commands::{
    delete_migrations, insert_migrations, migrations_by_mg_version_query,
    sqlite::{tables, CREATE_MIGRATION_TABLES},
  },
  Backend, BackendGeneric, Config, DbMigration, Migration, MigrationGroup,
};
use oapth_commons::Database;
use alloc::string::String;
use rusqlite::{Connection, Row};

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
        .query_map([], cb)?
        .into_iter()
        .collect::<Result<Vec<T>, _>>()?,
    )
  }
}

impl Backend for Rusqlite {}

#[async_trait::async_trait]
impl BackendGeneric for Rusqlite {
  #[oapth_macros::_dev_tools]
  #[inline]
  async fn clean(&mut self, buffer: &mut String) -> crate::Result<()> {
    crate::fixed_sql_commands::sqlite::clean(self, buffer).await
  }

  #[inline]
  async fn create_oapth_tables(&mut self) -> crate::Result<()> {
    self.execute(CREATE_MIGRATION_TABLES).await
  }

  #[inline]
  fn database() -> Database {
    Database::Sqlite
  }

  #[inline]
  async fn delete_migrations<S>(
    &mut self,
    buffer: &mut String,
    version: i32,
    mg: &MigrationGroup<S>,
  ) -> crate::Result<()>
  where
    S: AsRef<str> + Send + Sync
  {
    delete_migrations(self, buffer, mg, "", version).await
  }

  #[inline]
  async fn execute(&mut self, command: &str) -> crate::Result<()> {
    Ok(self.conn.execute_batch(command)?)
  }

  #[inline]
  async fn insert_migrations<'migration, DBS, I, S>(
    &mut self,
    buffer: &mut String,
    migrations: I,
    mg: &MigrationGroup<S>,
  ) -> crate::Result<()>
  where
    DBS: AsRef<[Database]> + 'migration,
    I: Clone + Iterator<Item = &'migration Migration<DBS, S>> + Send + Sync,
    S: AsRef<str> + Send + Sync + 'migration
  {
    insert_migrations(self, buffer, mg, "", migrations).await
  }

  #[inline]
  async fn migrations<S>(
    &mut self,
    buffer: &mut String,
    mg: &MigrationGroup<S>,
  ) -> crate::Result<Vec<DbMigration>>
  where
    S: AsRef<str> + Send + Sync
  {
    let fun = |e| {
      if let crate::Error::Rusqlite(inner) = e {
        inner
      } else {
        rusqlite::Error::InvalidQuery
      }
    };
    migrations_by_mg_version_query(buffer, mg.version(), "")?;
    let rslt = self.query(buffer.as_str(), |row| DbMigration::try_from(row).map_err(fun)).await;
    buffer.clear();
    rslt
  }

  #[inline]
    async fn query_string(
      &mut self,
      query: &str,
    ) -> crate::Result<Vec<String>>
  {
    self.query(query, |r| r.get::<_, String>(0)).await
  }

  #[inline]
  async fn tables(
    &mut self,
    schema: &str,
  ) -> crate::Result<Vec<String>>
  {
    let buffer = tables(schema)?;
    self.query(buffer.as_str(), |r| r.get::<_, String>(0)).await
  }

  #[inline]
  async fn transaction<I, S>(&mut self, commands: I) -> crate::Result<()>
  where
    I: Iterator<Item = S> + Send,
    S: AsRef<str> + Send + Sync,
  {
    let transaction = self.conn.transaction()?;
    for command in commands {
      transaction.execute_batch(command.as_ref())?;
    }
    transaction.commit()?;
    Ok(())
  }
}
