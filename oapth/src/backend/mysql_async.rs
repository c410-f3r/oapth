use crate::{
  fixed_sql_commands::{
    _delete_migrations, _insert_migrations, _migrations_by_group_version_query,
    _CREATE_MIGRATION_TABLES_MYSQL,
  },
  Backend, BoxFut, Config, DbMigration, Migration, MigrationGroup,
};
use core::convert::TryFrom;
use mysql_async::{prelude::Queryable, Conn, Params, Pool, Row, TxOpts};

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
  #[cfg_attr(feature = "_integration_tests", doc = "```rust")]
  #[cfg_attr(not(feature = "_integration_tests"), doc = "```ignore,rust")]
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

impl Backend for MysqlAsync {
  #[inline]
  fn create_oapth_tables<'a>(&'a mut self) -> BoxFut<'a, crate::Result<()>> {
    self.execute(_CREATE_MIGRATION_TABLES_MYSQL)
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
    self.transaction(command.split(';').filter(|el| !el.is_empty()))
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
      let vec: Vec<Row> =
        self.conn.query(_migrations_by_group_version_query(mg.version(), "")?.as_str()).await?;
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
        transaction.exec_drop(command.as_ref(), Params::Empty).await?;
      }
      transaction.commit().await?;
      Ok(())
    })
  }
}
