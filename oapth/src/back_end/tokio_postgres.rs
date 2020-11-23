use crate::{
  fixed_sql_commands::{
    _delete_migrations, _insert_migrations, _migrations_by_mg_version_query,
    postgres::{_all_tables, _CREATE_MIGRATION_TABLES},
  },
  BackEnd, BoxFut, DbMigration, Migration, MigrationGroup, _BackEnd, _OAPTH_SCHEMA_PREFIX,
};
use alloc::string::String;
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
  #[cfg_attr(feature = "_integration-tests", doc = "```rust")]
  #[cfg_attr(not(feature = "_integration-tests"), doc = "```ignore,rust")]
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

impl BackEnd for TokioPostgres {}

impl _BackEnd for TokioPostgres {
  #[inline]
  fn all_tables<'a>(&'a mut self, schema: &'a str) -> BoxFut<'a, crate::Result<Vec<String>>> {
    Box::pin(async move {
      let rows = self.conn.query(_all_tables(schema)?.as_str(), &[]).await?;
      rows.into_iter().map(|r| Ok(r.try_get::<_, String>(0)?)).collect::<crate::Result<_>>()
    })
  }

  #[cfg(feature = "dev-tools")]
  #[inline]
  fn clean<'a>(&'a mut self) -> BoxFut<'a, crate::Result<()>> {
    Box::pin(
      async move { Ok(self.execute(&crate::fixed_sql_commands::postgres::_clean()?).await?) },
    )
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
    Box::pin(async move { Ok(_delete_migrations(self, mg, _OAPTH_SCHEMA_PREFIX, version).await?) })
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
    Box::pin(_insert_migrations(self, mg, _OAPTH_SCHEMA_PREFIX, migrations))
  }

  #[inline]
  fn migrations<'a>(
    &'a mut self,
    mg: &'a MigrationGroup,
  ) -> BoxFut<'a, crate::Result<Vec<DbMigration>>> {
    Box::pin(async move {
      let buffer = _migrations_by_mg_version_query(mg.version(), _OAPTH_SCHEMA_PREFIX)?;
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
