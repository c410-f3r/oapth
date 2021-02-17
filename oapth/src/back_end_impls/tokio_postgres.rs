use alloc::string::String;
use core::{convert::TryFrom, str::FromStr};
use crate::{
  fixed_sql_commands::{
    delete_migrations, insert_migrations, migrations_by_mg_version_query,
    pg::{tables, CREATE_MIGRATION_TABLES},
  },MigrationGroupRef,
  BackEnd, BackEndGeneric, BoxFut, DbMigration, MigrationRef, OAPTH_SCHEMA_PREFIX
};
use oapth_commons::Database;
use native_tls::TlsConnector;
use postgres_native_tls::MakeTlsConnector;
use tokio_postgres::{Client, Config};

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
    let connector = MakeTlsConnector::new(TlsConnector::builder().build()?);
    let (client, conn) = config.connect(connector).await?;
    let _ = tokio::spawn(async move {
      if let Err(e) = conn.await {
        eprintln!("Connection error: {}", e);
      }
    });
    Ok(Self { conn: client })
  }
}

impl BackEnd for TokioPostgres {}

impl BackEndGeneric for TokioPostgres {
  #[oapth_macros::_dev_tools]
  #[inline]
  fn clean<'a, 'ret>(&'a mut self) -> BoxFut<'ret, crate::Result<()>>
  where
    'a: 'ret,
    Self: 'ret,
  {
    Box::pin(crate::fixed_sql_commands::pg::clean(self))
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
    Database::Pg
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
    Box::pin(delete_migrations(self, mg, OAPTH_SCHEMA_PREFIX, version))
  }

  #[inline]
  fn execute<'a, 'b, 'ret>(&'a mut self, command: &'b str) -> BoxFut<'ret, crate::Result<()>>
  where
    'a: 'ret,
    'b: 'ret,
    Self: 'ret,
  {
    Box::pin(async move { Ok(self.conn.batch_execute(command).await?) })
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
    Box::pin(insert_migrations(self, mg, OAPTH_SCHEMA_PREFIX, migrations))
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
      let buffer = migrations_by_mg_version_query(mg.version(), OAPTH_SCHEMA_PREFIX)?;
      let vec = self.conn.query(buffer.as_str(), &[]).await?;
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
      let rows = self.conn.query(query, &[]).await?;
      rows.into_iter().map(|r| Ok(r.try_get::<_, String>(0)?)).collect::<crate::Result<_>>()
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
      let rows = self.conn.query(tables(schema)?.as_str(), &[]).await?;
      rows.into_iter().map(|r| Ok(r.try_get::<_, String>(0)?)).collect::<crate::Result<_>>()
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
      let transaction = self.conn.transaction().await?;
      for command in commands {
        transaction.batch_execute(command.as_ref()).await?;
      }
      transaction.commit().await?;
      Ok(())
    })
  }
}
