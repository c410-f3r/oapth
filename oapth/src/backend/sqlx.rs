use crate::{
  fixed_sql_commands::{
    _delete_migrations, _insert_migrations, _migrations_by_group_version_query,
  },
  Backend, BoxFut, Config, DbMigration, Migration, MigrationGroup,
};
use core::convert::TryFrom;
use sqlx_core::{connection::Connection, executor::Executor};

macro_rules! create_sqlx_backend {
  (
    $(#[$new_doc:meta])+ $backend_name:ident,
    $conn_ty:ty,
    $create_oapth_tables:expr,
    $insert_migrations:ident($schema:expr)
  ) => {
    /// Wraps functionalities for the `sqlx` crate
    #[derive(Debug)]
    pub struct $backend_name {
      conn: $conn_ty,
    }

    impl $backend_name {
      $(#[$new_doc])+
      #[inline]
      pub async fn new(config: &Config) -> crate::Result<Self> {
        let conn = <$conn_ty>::connect(config.url()).await?;
        Ok(Self { conn })
      }
    }

    impl Backend for $backend_name {
      #[inline]
      fn create_oapth_tables<'a>(&'a mut self) -> BoxFut<'a, crate::Result<()>> {
        self.execute($create_oapth_tables)
      }

      #[inline]
      fn delete_migrations<'a>(
        &'a mut self,
        version: i32,
        mg: &'a MigrationGroup,
      ) -> BoxFut<'a, crate::Result<()>> {
        Box::pin(async move { Ok(_delete_migrations(self, mg, $schema, version).await?) })
      }

      #[inline]
      fn execute<'a>(&'a mut self, command: &'a str) -> BoxFut<'a, crate::Result<()>> {
        Box::pin(async move { Ok(self.conn.execute(command).await.map(|_| {})?) })
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
        Box::pin(_insert_migrations(self, mg, $schema, migrations))
      }

      #[inline]
      fn migrations<'a>(&'a mut self, mg: &'a MigrationGroup,) -> BoxFut<'a, crate::Result<Vec<DbMigration>>> {
        use futures::{StreamExt, TryStreamExt};
        Box::pin(async move {
          let query = _migrations_by_group_version_query(mg.version(), $schema)?;
          let rows = sqlx_core::query::query(query.as_str()).fetch(&mut self.conn);
          Ok(
            rows
              .map(|el| DbMigration::try_from(el.map_err(crate::Error::Sqlx)?))
              .try_collect::<Vec<_>>()
              .await?,
          )
        })
      }

      #[inline]
      fn transaction<'a, I, S>(&'a mut self, commands: I) -> BoxFut<'a, crate::Result<()>>
      where
        I: Iterator<Item = S> + 'a,
        S: AsRef<str>,
      {
        Box::pin(async move {
          let mut transaction = self.conn.begin().await?;
          for command in commands {
            transaction.execute(command.as_ref()).await?;
          }
          transaction.commit().await?;
          Ok(())
        })
      }
    }
  };
}

#[cfg(feature = "with-sqlx-mssql")]
create_sqlx_backend!(
  /// Creates a new instance from all necessary parameters.
  ///
  /// # Example
  ///
  #[cfg_attr(feature = "_integration_tests", doc = "```rust")]
  #[cfg_attr(not(feature = "_integration_tests"), doc = "```ignore,rust")]
  /// # #[tokio::main] async fn main() -> oapth::Result<()> {
  /// use oapth::{Config, SqlxMssql};
  /// let _ = SqlxMssql::new(&Config::with_url_from_default_var()?).await?;
  /// # Ok(()) }
  SqlxMssql,
  sqlx_core::mssql::MssqlConnection,
  crate::fixed_sql_commands::_CREATE_MIGRATION_TABLES_MSSQL,
  _insert_migrations(crate::_OAPTH_SCHEMA)
);

#[cfg(feature = "with-sqlx-mysql")]
create_sqlx_backend!(
  /// Creates a new instance from all necessary parameters.
  ///
  /// # Example
  ///
  #[cfg_attr(feature = "_integration_tests", doc = "```rust")]
  #[cfg_attr(not(feature = "_integration_tests"), doc = "```ignore,rust")]
  /// # #[tokio::main] async fn main() -> oapth::Result<()> {
  /// use oapth::{Config, SqlxMysql};
  /// let _ = SqlxMysql::new(&Config::with_url_from_default_var()?).await?;
  /// # Ok(()) }
  SqlxMysql,
  sqlx_core::mysql::MySqlConnection,
  crate::fixed_sql_commands::_CREATE_MIGRATION_TABLES_MYSQL,
  _insert_migrations("")
);

#[cfg(feature = "with-sqlx-postgres")]
create_sqlx_backend!(
  /// Creates a new instance from all necessary parameters.
  ///
  /// # Example
  ///
  #[cfg_attr(feature = "_integration_tests", doc = "```rust")]
  #[cfg_attr(not(feature = "_integration_tests"), doc = "```ignore,rust")]
  /// # #[tokio::main] async fn main() -> oapth::Result<()> {
  /// use oapth::{Config, SqlxPostgres};
  /// let _ = SqlxPostgres::new(&Config::with_url_from_default_var()?).await?;
  /// # Ok(()) }
  SqlxPostgres,
  sqlx_core::postgres::PgConnection,
  crate::fixed_sql_commands::_CREATE_MIGRATION_TABLES_POSTGRESQL,
  _insert_migrations(crate::_OAPTH_SCHEMA)
);

#[cfg(feature = "with-sqlx-sqlite")]
create_sqlx_backend!(
  /// Creates a new instance from all necessary parameters.
  ///
  /// # Example
  ///
  #[cfg_attr(feature = "_integration_tests", doc = "```rust")]
  #[cfg_attr(not(feature = "_integration_tests"), doc = "```ignore,rust")]
  /// # #[tokio::main] async fn main() -> oapth::Result<()> {
  /// use oapth::{Config, SqlxSqlite};
  /// let _ = SqlxSqlite::new(&Config::with_url_from_default_var()?).await?;
  /// # Ok(()) }
  SqlxSqlite,
  sqlx_core::sqlite::SqliteConnection,
  crate::fixed_sql_commands::_CREATE_MIGRATION_TABLES_SQLITE,
  _insert_migrations("")
);
