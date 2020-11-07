use crate::{
  fixed_sql_commands::{
    _insert_migrations, _migrations_by_group_version_query, _CREATE_MIGRATION_TABLES_POSTGRESQL,
  },
  Backend, BoxFut, DbMigration, Migration, MigrationGroup, _OAPTH_SCHEMA,
};
use core::{convert::TryFrom, str::FromStr};
use tokio_postgres::{Client, Config, NoTls};

/// Wraps functionalities for the `tokio-postgres` crate
#[derive(Debug)]
pub struct TokioPostgres {
  conn: Client,
}

impl TokioPostgres {
  /// Creates a new instance from all necessary parameters.
  ///
  /// # Example
  ///
  #[cfg_attr(feature = "_integration_tests", doc = "```rust")]
  #[cfg_attr(not(feature = "_integration_tests"), doc = "```ignore,rust")]
  /// # #[tokio::main] async fn main() -> oapth::Result<()> {
  /// use oapth::{Config, TokioPostgres};
  /// let _ = TokioPostgres::new(&Config::with_url_from_default_var()?).await?;
  /// # Ok(()) }
  /// ```
  #[inline]
  pub async fn new(oapth_config: &crate::Config) -> crate::Result<Self> {
    let config = Config::from_str(oapth_config.url())?;
    let (client, conn) = config.connect(NoTls).await?;
    tokio::spawn(async move {
      if let Err(e) = conn.await {
        eprintln!("Connection error: {}", e);
      }
    });
    Ok(Self { conn: client })
  }
}

impl Backend for TokioPostgres {
  #[inline]
  fn create_oapth_tables<'a>(&'a mut self) -> BoxFut<'a, crate::Result<()>> {
    self.execute(_CREATE_MIGRATION_TABLES_POSTGRESQL)
  }

  #[inline]
  fn execute<'a>(&'a mut self, command: &'a str) -> BoxFut<'a, crate::Result<()>> {
    Box::pin(async move { Ok(self.conn.batch_execute(command).await?) })
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
    Box::pin(_insert_migrations(self, mg, _OAPTH_SCHEMA, migrations))
  }

  #[inline]
  fn migrations<'a>(
    &'a mut self,
    mg: &'a MigrationGroup,
  ) -> BoxFut<'a, crate::Result<Vec<DbMigration>>> {
    Box::pin(async move {
      let buffer = _migrations_by_group_version_query(mg.version(), _OAPTH_SCHEMA)?;
      let vec = self.conn.query(buffer.as_str(), &[]).await?;
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
      let transaction = self.conn.transaction().await?;
      for command in commands {
        transaction.batch_execute(command.as_ref()).await?;
      }
      transaction.commit().await?;
      Ok(())
    })
  }
}
