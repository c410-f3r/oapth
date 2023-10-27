use crate::{database::Database, DatabaseTy};
use futures::{AsyncRead, AsyncWrite, TryStreamExt};
use tiberius::{Client, Row};

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
  /// use oapth::{sm::Config, database::Tiberius};
  /// use tokio_util::compat::TokioAsyncWriteCompatExt;
  /// let c = Config::with_url_from_default_var().unwrap();
  /// let tcp = tokio::net::TcpStream::connect(c.full_host().unwrap()).await.unwrap();
  /// let _ = Tiberius::new(&c, tcp.compat_write()).await.unwrap();
  /// # Ok(()) }
  /// ```
  #[cfg(feature = "sm")]
  #[inline]
  pub async fn new(oapth_config: &crate::sm::Config, tcp: T) -> crate::Result<Self> {
    let mut config = tiberius::Config::new();
    config.authentication(tiberius::AuthMethod::sql_server(
      oapth_config.user()?,
      oapth_config.password()?,
    ));
    config.host(oapth_config.host()?);
    config.port(oapth_config.port()?);
    Self::manage_trust_server_certificate(&mut config, oapth_config.url());
    let conn = Client::connect(config, tcp).await?;
    Ok(Self { conn })
  }

  #[cfg(feature = "sm")]
  #[inline]
  fn manage_trust_server_certificate(c: &mut tiberius::Config, url: &str) {
    let opt = || url.split("trustServerCertificate=").nth(1)?.parse::<bool>().ok();
    if let Some(e) = opt() {
      if e {
        c.trust_cert();
      }
    }
  }
}

impl<T> Database for Tiberius<T>
where
  T: AsyncRead + AsyncWrite + Send + Unpin,
{
  const TY: DatabaseTy = DatabaseTy::Mssql;

  type Row = Row;

  #[inline]
  async fn execute(&mut self, cmd: &str) -> crate::Result<()> {
    Ok(self.conn.execute(cmd, &[][..]).await.map(|_| ())?)
  }

  #[inline]
  async fn row(&mut self, cmd: &str) -> crate::Result<Self::Row> {
    let query_result = self.conn.query(cmd, &[]).await?;
    query_result.into_row().await?.ok_or(crate::Error::InvalidSqlQuery)
  }

  #[inline]
  async fn rows<E>(
    &mut self,
    cmd: &str,
    mut cb: impl FnMut(Self::Row) -> Result<(), E>,
  ) -> Result<(), E>
  where
    E: From<crate::Error>,
  {
    let query_result = self.conn.query(cmd, &[]).await.map_err(From::from)?;
    let mut stream = query_result.into_row_stream();
    while let Some(row) = stream.try_next().await.map_err(From::from)? {
      cb(row)?;
    }
    Ok(())
  }

  #[inline]
  async fn transaction(&mut self, cmd: &str) -> crate::Result<()> {
    self.execute(cmd).await?;
    Ok(())
  }
}

#[cfg(feature = "sm")]
impl<T> crate::sm::SchemaManagement for Tiberius<T>
where
  T: AsyncRead + AsyncWrite + Send + Unpin,
{
  #[inline]
  async fn clear(
    &mut self,
    buffer: (&mut String, &mut Vec<crate::Identifier>),
  ) -> crate::Result<()> {
    crate::sm::fixed_sql_commands::mssql::_clear(buffer, self).await
  }

  #[inline]
  async fn create_oapth_tables(&mut self) -> crate::Result<()> {
    self.execute(crate::sm::fixed_sql_commands::mssql::_CREATE_MIGRATION_TABLES).await
  }

  #[inline]
  async fn delete_migrations<S>(
    &mut self,
    buffer_cmd: &mut String,
    mg: &crate::sm::MigrationGroup<S>,
    version: i32,
  ) -> crate::Result<()>
  where
    S: AsRef<str>,
  {
    crate::sm::fixed_sql_commands::_delete_migrations(
      buffer_cmd,
      self,
      mg,
      crate::sm::_OAPTH_SCHEMA_PREFIX,
      version,
    )
    .await
  }

  #[inline]
  async fn insert_migrations<'migration, DBS, I, S>(
    &mut self,
    buffer_cmd: &mut String,
    mg: &crate::sm::MigrationGroup<S>,
    migrations: I,
  ) -> crate::Result<()>
  where
    DBS: AsRef<[DatabaseTy]> + 'migration,
    I: Clone + Iterator<Item = &'migration crate::sm::UserMigration<DBS, S>>,
    S: AsRef<str> + 'migration,
  {
    crate::sm::fixed_sql_commands::_insert_migrations(
      buffer_cmd,
      self,
      mg,
      migrations,
      crate::sm::_OAPTH_SCHEMA_PREFIX,
    )
    .await
  }

  #[inline]
  async fn migrations<S>(
    &mut self,
    buffer_cmd: &mut String,
    mg: &crate::sm::MigrationGroup<S>,
    results: &mut Vec<crate::sm::DbMigration>,
  ) -> crate::Result<()>
  where
    S: AsRef<str>,
  {
    crate::sm::fixed_sql_commands::_migrations_by_mg_version_query(
      buffer_cmd,
      self,
      mg.version(),
      results,
      crate::sm::_OAPTH_SCHEMA_PREFIX,
    )
    .await
  }

  #[inline]
  async fn table_names(
    &mut self,
    buffer_cmd: &mut String,
    results: &mut Vec<crate::Identifier>,
    schema: &str,
  ) -> crate::Result<()> {
    crate::sm::fixed_sql_commands::mssql::_table_names(buffer_cmd, self, results, schema).await
  }
}
