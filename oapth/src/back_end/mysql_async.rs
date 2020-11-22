use crate::{
  fixed_sql_commands::{
    _delete_migrations, _insert_migrations, _migrations_by_mg_version_query,
    mysql::{_all_tables, _CREATE_MIGRATION_TABLES},
  },
  BackEnd, BoxFut, Config, DbMigration, Migration, MigrationGroup, _BackEnd,
};
use alloc::string::String;
use core::convert::TryFrom;
use mysql_async::{prelude::Queryable, Conn, Pool, Row, TxOpts};

/// Wraps functionalities for the `mysql_async` crate
#[derive(Debug)]
pub struct MysqlAsync {
  conn: Conn,
}

impl MysqlAsync {
  /// Creates a new instance from all necessary parameters.
  ///
  /// # Example
  ///
  #[cfg_attr(feature = "_integration-tests", doc = "```rust")]
  #[cfg_attr(not(feature = "_integration-tests"), doc = "```ignore,rust")]
  /// # #[tokio::main] async fn main() -> oapth::Result<()> {
  /// use oapth::{Config, MysqlAsync};
  /// let _ = MysqlAsync::new(&Config::with_url_from_default_var()?).await?;
  /// # Ok(()) }
  /// ```
  #[inline]
  pub async fn new(config: &Config) -> crate::Result<Self> {
    let pool = Pool::new(config.url());
    let conn = pool.get_conn().await?;
    Ok(Self { conn })
  }
}

impl BackEnd for MysqlAsync {}

impl _BackEnd for MysqlAsync {
  #[inline]
  fn all_tables<'a>(&'a mut self, schema: &'a str) -> BoxFut<'a, crate::Result<Vec<String>>> {
    Box::pin(async move {
      let buffer = _all_tables(schema)?;
      let rows: Vec<Row> = self.conn.query(buffer.as_str()).await?;
      rows
        .into_iter()
        .map(|row| {
          row.get::<String, _>(0).ok_or(crate::Error::MysqlAsync(mysql_async::Error::Driver(
            mysql_async::DriverError::FromRow { row },
          )))
        })
        .collect::<crate::Result<_>>()
    })
  }

  #[cfg(feature = "dev-tools")]
  #[inline]
  fn clean<'a>(&'a mut self) -> BoxFut<'a, crate::Result<()>> {
    Box::pin(async move { Ok(self.execute(&crate::fixed_sql_commands::mysql::_clean()?).await?) })
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
    Box::pin(async move { Ok(self.conn.query_drop(command).await?) })
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
      let buffer = _migrations_by_mg_version_query(mg.version(), "")?;
      let vec: Vec<Row> = self.conn.query(buffer.as_str()).await?;
      vec.into_iter().map(DbMigration::try_from).collect::<crate::Result<Vec<_>>>()
    })
  }

  #[inline]
  fn transaction<'a, I, S>(&'a mut self, commands: I) -> BoxFut<'a, crate::Result<()>>
  where
    I: Iterator<Item = S> + 'a,
    S: AsRef<str>,
  {
    Box::pin(async move {
      let mut transaction = self.conn.start_transaction(TxOpts::default()).await?;
      for command in commands {
        transaction.query_drop(command.as_ref()).await?;
      }
      transaction.commit().await?;
      Ok(())
    })
  }
}
