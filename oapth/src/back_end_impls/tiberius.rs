use crate::{
  fixed_sql_commands::{
    delete_migrations, insert_migrations, migrations_by_mg_version_query,
    mssql::{tables, CREATE_MIGRATION_TABLES},
  },
  BackEnd, BackEndGeneric, BoxFut, DbMigration, MigrationRef, MigrationGroupRef, OAPTH_SCHEMA_PREFIX,
};
use alloc::string::String;
use oapth_commons::Database;
use core::convert::TryFrom;
use futures::{AsyncRead, AsyncWrite};
use tiberius::{AuthMethod, Client, Config};

/// Wraps functionalities for the `tiberius` crate
///
/// This BackEnd currently doesn't support transactions
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

impl<T> BackEnd for Tiberius<T>
where
  T: AsyncRead + AsyncWrite + Send + Unpin
{}

impl<T> BackEndGeneric for Tiberius<T>
where
  T: AsyncRead + AsyncWrite + Send + Unpin,
{
  #[oapth_macros::_dev_tools]
  #[inline]
  fn clean<'a, 'ret>(&'a mut self) -> BoxFut<'ret, crate::Result<()>>
  where
    'a: 'ret,
    Self: 'ret,
  {
    Box::pin(crate::fixed_sql_commands::mssql::clean(self))
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
    Database::Mssql
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
    Box::pin(async move { Ok(self.conn.execute(command, &[][..]).await.map(|_| ())?) })
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
      let vec = self.conn.query(buffer.as_str(), &[]).await?.into_first_result().await?;
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
      let query_result = self.conn.query(query, &[]).await?;
      let rows = query_result.into_first_result().await?;
      rows
        .into_iter()
        .map(|r| {
          let opt = r.try_get::<&str, _>(0)?;
          opt.map(|el| el.into()).ok_or(crate::Error::Other("Invalid query"))
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
      let query_result = self.conn.query(tables(schema)?.as_str(), &[]).await?;
      let rows = query_result.into_first_result().await?;
      rows
        .into_iter()
        .map(|r| {
          let opt = r.try_get::<&str, _>(0)?;
          opt.map(|el| el.into()).ok_or(crate::Error::Other("Invalid query"))
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
      for command in commands {
        self.execute(command.as_ref()).await?;
      }
      Ok(())
    })
  }
}
