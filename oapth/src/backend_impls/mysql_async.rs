use crate::{
  fixed_sql_commands::{
    delete_migrations, insert_migrations, migrations_by_mg_version_query,
    mysql::{tables, CREATE_MIGRATION_TABLES},
  },
  Backend, BackendGeneric, Config, DbMigration, Migration, MigrationGroup,
};
use oapth_commons::Database;
use alloc::string::String;
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

impl Backend for MysqlAsync {}

#[async_trait::async_trait]
impl BackendGeneric for MysqlAsync {
  #[oapth_macros::_dev_tools]
  #[inline]
  async fn clean(&mut self, buffer: &mut String) -> crate::Result<()> {
    crate::fixed_sql_commands::mysql::clean(self, buffer).await
  }

  #[inline]
  async fn create_oapth_tables(&mut self) -> crate::Result<()> {
    self.execute(CREATE_MIGRATION_TABLES).await
  }

  #[inline]
  fn database() -> Database {
    Database::Mysql
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
      if command.is_empty() {
        Ok(())
      }
      else {
        Ok(self.conn.query_drop(command).await?)
      }
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
    migrations_by_mg_version_query(buffer, mg.version(), "")?;
    let vec: Vec<Row> = self.conn.query(buffer.as_str()).await?;
    let rslt = vec.into_iter().map(DbMigration::try_from).collect::<crate::Result<Vec<_>>>();
    buffer.clear();
    rslt
  }

  #[inline]
  async fn query_string(&mut self, query: &str) -> crate::Result<Vec<String>> {
      let rows: Vec<Row> = self.conn.query(query).await?;
      rows
        .into_iter()
        .map(|row| {
          row.get::<String, _>(0).ok_or_else(|| crate::Error::MysqlAsync(mysql_async::Error::Driver(
            mysql_async::DriverError::FromRow { row },
          ).into()))
        })
        .collect::<crate::Result<_>>()
  }
  
  #[inline]
  async fn tables(&mut self, schema: &str) -> crate::Result<Vec<String>> {
    let buffer = tables(schema)?;
      let rows: Vec<Row> = self.conn.query(buffer.as_str()).await?;
      rows
        .into_iter()
        .map(|row| {
          row.get::<String, _>(0).ok_or_else(|| crate::Error::MysqlAsync(mysql_async::Error::Driver(
            mysql_async::DriverError::FromRow { row },
          ).into()))
        })
        .collect::<crate::Result<_>>()
  }

  #[inline]
  async fn transaction<I, S>(&mut self, commands: I) -> crate::Result<()>
  where
    I: Iterator<Item = S> + Send,
    S: AsRef<str> + Send + Sync,
  {
      let mut transaction = self.conn.start_transaction(TxOpts::default()).await?;
      for command in commands {
        transaction.query_drop(command.as_ref()).await?;
      }
      transaction.commit().await?;
      Ok(())
  }
}
