use crate::{
  fixed_sql_commands::{
    _delete_migrations, _insert_migrations, _migrations_by_mg_version_query,
    mssql::{_all_tables, _CREATE_MIGRATION_TABLES},
  },
  BackEnd, BoxFut, DbMigration, Migration, MigrationGroup, _BackEnd, _OAPTH_SCHEMA,
};
use alloc::string::String;
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
  /// use tokio_util::compat::Tokio02AsyncWriteCompatExt;
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
    let conn = Client::connect(config, tcp).await?;
    Ok(Self { conn })
  }
}

impl<T> BackEnd for Tiberius<T> where T: AsyncRead + AsyncWrite + Send + Unpin {}

impl<T> _BackEnd for Tiberius<T>
where
  T: AsyncRead + AsyncWrite + Send + Unpin,
{
  #[inline]
  fn all_tables<'a>(&'a mut self, schema: &'a str) -> BoxFut<'a, crate::Result<Vec<String>>> {
    Box::pin(async move {
      let query_result = self.conn.query(_all_tables(schema)?.as_str(), &[]).await?;
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

  #[cfg(feature = "dev-tools")]
  #[inline]
  fn clean<'a>(&'a mut self) -> BoxFut<'a, crate::Result<()>> {
    Box::pin(async move { Ok(self.execute(&crate::fixed_sql_commands::mssql::_clean()?).await?) })
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
    Box::pin(async move { Ok(_delete_migrations(self, mg, _OAPTH_SCHEMA, version).await?) })
  }

  #[inline]
  fn execute<'a>(&'a mut self, command: &'a str) -> BoxFut<'a, crate::Result<()>> {
    Box::pin(async move { Ok(self.conn.execute(command, &[][..]).await.map(|_| ())?) })
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
      let buffer = _migrations_by_mg_version_query(mg.version(), _OAPTH_SCHEMA)?;
      let vec = self.conn.query(buffer.as_str(), &[]).await?.into_first_result().await?;
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
      for command in commands {
        self.execute(command.as_ref()).await?;
      }
      Ok(())
    })
  }
}
