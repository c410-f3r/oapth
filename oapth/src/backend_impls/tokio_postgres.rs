use alloc::string::String;
use core::str::FromStr;
use crate::{
  fixed_sql_commands::{
    delete_migrations, insert_migrations, migrations_by_mg_version_query,
    pg::{tables, CREATE_MIGRATION_TABLES},
  },MigrationGroup,
  Backend, BackendGeneric,  DbMigration,  OAPTH_SCHEMA_PREFIX, Migration
};
use std::fs;
use oapth_commons::Database;
use native_tls::{Certificate, TlsConnector, TlsConnectorBuilder};
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
    let mut tcb = TlsConnector::builder();
    let actual_oapth_config = Self::manage_sslrootcert(oapth_config.url(), &mut tcb)?;
    let config = Config::from_str(actual_oapth_config.url())?;
    let connector = MakeTlsConnector::new(tcb.build()?);
    let (client, conn) = config.connect(connector).await?;
    let _ = tokio::spawn(async move {
      if let Err(e) = conn.await {
        eprintln!("Connection error: {}", e);
      }
    });
    Ok(Self { conn: client })
  }

  // tokio-postgres triggers an error when sslrootcert is present, threfore, a new "splitted"
  // instance is necessary
  #[inline]
  fn manage_sslrootcert(orig_url: &str, tcb: &mut TlsConnectorBuilder) -> crate::Result<crate::Config> {
    let mut url = String::new();
    let mut first_iter = orig_url.split("sslrootcert=");
    if let Some(before_sslrootcert) = first_iter.next() {
      url.push_str(before_sslrootcert);
      if let Some(after_sslrootcert) = first_iter.next() {
        let mut second_iter = after_sslrootcert.split('&');
        if let Some(before_first_ampersand) = second_iter.next() {
          let read_rslt = fs::read(before_first_ampersand);
          let cert = read_rslt.map_err(|e| crate::Error::OapthCommons(e.into()))?;
          let root_ca = Certificate::from_pem(&cert)?;
          let _ = tcb.add_root_certificate(root_ca);
        }
        if let Some(after_first_ampersand) = second_iter.next() {
          url.push_str(after_first_ampersand);
        }
        for s in second_iter {
          url.push('&');
          url.push_str(s);
        }
      }
    }
    else {
      url = orig_url.into();
    }
    Ok(crate::Config::with_url(url))
  }
}

impl Backend for TokioPostgres {}

#[async_trait::async_trait]
impl BackendGeneric for TokioPostgres {
  #[oapth_macros::_dev_tools]
  #[inline]
  async fn clean(&mut self, buffer: &mut String) -> crate::Result<()> {
    crate::fixed_sql_commands::pg::clean(self, buffer).await
  }

  #[inline]
  async fn create_oapth_tables(&mut self) -> crate::Result<()> {
    self.execute(CREATE_MIGRATION_TABLES).await
  }

  #[inline]
  fn database() -> Database {
    Database::Pg
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
    delete_migrations(self, buffer, mg, OAPTH_SCHEMA_PREFIX, version).await
  }

  #[inline]
  async fn execute(&mut self, command: &str) -> crate::Result<()> {
    Ok(self.conn.batch_execute(command).await?)
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
    insert_migrations(self, buffer, mg, OAPTH_SCHEMA_PREFIX, migrations).await
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
    migrations_by_mg_version_query(buffer, mg.version(), OAPTH_SCHEMA_PREFIX)?;
    let vec = self.conn.query(buffer, &[]).await?;
    let rslt = vec.into_iter().map(DbMigration::try_from).collect::<crate::Result<Vec<_>>>();
    buffer.clear();
    rslt
  }

  #[inline]
    async fn query_string(
      &mut self,
      query: &str,
    ) -> crate::Result<Vec<String>>
  {
    let rows = self.conn.query(query, &[]).await?;
    rows.into_iter().map(|r| Ok(r.try_get::<_, String>(0)?)).collect::<crate::Result<_>>()
  }

  #[inline]
  async fn tables(
    &mut self,
    schema: &str,
  ) -> crate::Result<Vec<String>>
  {
    let rows = self.conn.query(tables(schema)?.as_str(), &[]).await?;
    rows.into_iter().map(|r| Ok(r.try_get::<_, String>(0)?)).collect::<crate::Result<_>>()
  }

  #[inline]
  async fn transaction<I, S>(&mut self, commands: I) -> crate::Result<()>
  where
    I: Iterator<Item = S> + Send,
    S: AsRef<str> + Send + Sync,
  {
    let transaction = self.conn.transaction().await?;
    for command in commands {
      transaction.batch_execute(command.as_ref()).await?;
    }
    transaction.commit().await?;
    Ok(())
  }
}
