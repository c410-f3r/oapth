use crate::{
  fixed_sql_commands::{
    delete_migrations, insert_migrations, migrations_by_mg_version_query,
    mssql::{tables, CREATE_MIGRATION_TABLES},
  },
  Backend, BackendGeneric,  DbMigration,  MigrationGroup, OAPTH_SCHEMA_PREFIX, Migration,
};
use alloc::string::String;
use oapth_commons::Database;
use futures::{AsyncRead, AsyncWrite};
use tiberius::{AuthMethod, Client, Config};

/// Wraps functionalities for the `tiberius` crate
///
/// This Backend currently doesn't support transactions
#[derive(Debug)]
pub struct Tiberius<T>
where
  T: AsyncRead + AsyncWrite + Send + Unpin,
{
  conn: Client<T>,
}

impl<T> Tiberius<T>
where
  T: AsyncRead + AsyncWrite + Send + Unpin,
{
  /// Creates a new instance from all necessary parameters.
  ///
  /// # Example
  ///
  #[cfg_attr(feature = "_integration-tests", doc = "```rust")]
  #[cfg_attr(not(feature = "_integration-tests"), doc = "```ignore,rust")]
  /// # #[tokio::main] async fn main() -> oapth::Result<()> {
  /// use oapth::{Config, Tiberius};
  /// use tokio_util::compat::TokioAsyncWriteCompatExt;
  /// let c = Config::with_url_from_default_var().unwrap();
  /// let tcp = tokio::net::TcpStream::connect(c.full_host().unwrap()).await.unwrap();
  /// let _ = Tiberius::new(&c, tcp.compat_write()).await.unwrap();
  /// # Ok(()) }
  /// ```
  #[inline]
  pub async fn new(oapth_config: &crate::Config, tcp: T) -> crate::Result<Self> {
    let mut config = Config::new();
    config.authentication(AuthMethod::sql_server(oapth_config.user()?, oapth_config.password()?));
    config.host(oapth_config.host()?);
    config.port(oapth_config.port()?);
    Self::manage_trust_server_certificate(&mut config, oapth_config.url());
    let conn = Client::connect(config, tcp).await?;
    Ok(Self { conn })
  }

  #[inline]
  fn manage_trust_server_certificate(c: &mut Config, url: &str) {
    let opt = || url.split("trustServerCertificate=").nth(1)?.parse::<bool>().ok();
    if let Some(e) = opt() {
      if e {
        c.trust_cert();
      }
    }
  }
}

impl<T> Backend for Tiberius<T>
where
  T: AsyncRead + AsyncWrite + Send + Unpin
{}

#[async_trait::async_trait]
impl<T> BackendGeneric for Tiberius<T>
where
  T: AsyncRead + AsyncWrite + Send + Unpin,
{
  #[oapth_macros::_dev_tools]
  #[inline]
  async fn clean(&mut self, buffer: &mut String) -> crate::Result<()> {
    crate::fixed_sql_commands::mssql::clean(self, buffer).await
  }

  #[inline]
  async fn create_oapth_tables(&mut self) -> crate::Result<()> {
    self.execute(CREATE_MIGRATION_TABLES).await
  }

  #[inline]
  fn database() -> Database {
    Database::Mssql
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
    Ok(self.conn.execute(command, &[][..]).await.map(|_| ())?)
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
    let vec = self.conn.query(buffer.as_str(), &[]).await?.into_first_result().await?;
    buffer.clear();
    vec.into_iter().map(DbMigration::try_from).collect::<crate::Result<Vec<_>>>()
  }

  #[inline]
    async fn query_string(
      &mut self,
      query: &str,
    ) -> crate::Result<Vec<String>>
  {
    let query_result = self.conn.query(query, &[]).await?;
    let rows = query_result.into_first_result().await?;
    rows
      .into_iter()
      .map(|r| {
        let opt = r.try_get::<&str, _>(0)?;
        opt.map(|el| el.into()).ok_or(crate::Error::Other("Invalid query"))
      })
      .collect::<crate::Result<_>>()
  }

  #[inline]
  async fn tables(
    &mut self,
    schema: &str,
  ) -> crate::Result<Vec<String>>
  {
    let query_result = self.conn.query(tables(schema)?.as_str(), &[]).await?;
    let rows = query_result.into_first_result().await?;
    rows
      .into_iter()
      .map(|r| {
        let opt = r.try_get::<&str, _>(0)?;
        opt.map(|el| el.into()).ok_or(crate::Error::Other("Invalid query"))
      })
      .collect::<crate::Result<_>>()
  }

  #[inline]
  async fn transaction<I, S>(&mut self, commands: I) -> crate::Result<()>
  where
    I: Iterator<Item = S> + Send,
    S: AsRef<str> + Send + Sync,
  {
    for command in commands {
      self.execute(command.as_ref()).await?;
    }
    Ok(())
  }
}
