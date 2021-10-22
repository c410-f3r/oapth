use crate::{
  fixed_sql_commands::{
    delete_migrations, insert_migrations, migrations_by_mg_version_query,
    mysql::{tables, CREATE_MIGRATION_TABLES},
  },
  BackEnd, BackEndGeneric, BoxFut, Config, DbMigration, MigrationRef, MigrationGroupRef,
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

impl BackEnd for MysqlAsync {}

impl BackEndGeneric for MysqlAsync {
  #[oapth_macros::_dev_tools]
  #[inline]
  fn clean<'a, 'ret>(&'a mut self) -> BoxFut<'ret, crate::Result<()>>
  where
    'a: 'ret,
    Self: 'ret,
  {
    Box::pin(crate::fixed_sql_commands::mysql::clean(self))
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
    Database::Mysql
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
    Box::pin(async move {
      if command.is_empty() {
        Ok(())
      }
      else {
        Ok(self.conn.query_drop(command).await?)
      }
    })
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
      let buffer = migrations_by_mg_version_query(mg.version(), "")?;
      let vec: Vec<Row> = self.conn.query(buffer.as_str()).await?;
      vec.into_iter().map(DbMigration::try_from).collect::<crate::Result<Vec<_>>>()
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
    Box::pin(async move {
      let rows: Vec<Row> = self.conn.query(query).await?;
      rows
        .into_iter()
        .map(|row| {
          row.get::<String, _>(0).ok_or_else(|| crate::Error::MysqlAsync(mysql_async::Error::Driver(
            mysql_async::DriverError::FromRow { row },
          ).into()))
        })
        .collect::<crate::Result<_>>()
    })
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
      let rows: Vec<Row> = self.conn.query(buffer.as_str()).await?;
      rows
        .into_iter()
        .map(|row| {
          row.get::<String, _>(0).ok_or_else(|| crate::Error::MysqlAsync(mysql_async::Error::Driver(
            mysql_async::DriverError::FromRow { row },
          ).into()))
        })
        .collect::<crate::Result<_>>()
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
      let mut transaction = self.conn.start_transaction(TxOpts::default()).await?;
      for command in commands {
        transaction.query_drop(command.as_ref()).await?;
      }
      transaction.commit().await?;
      Ok(())
    })
  }
}
