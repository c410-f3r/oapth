use crate::{
  fixed_sql_commands::{_delete_migrations, _insert_migrations, _migrations_by_mg_version_query},
  BackEnd, BoxFut, Config, DbMigration, Migration, MigrationGroup, _BackEnd,
};
use alloc::string::String;
use core::convert::TryFrom;
use futures::{StreamExt, TryStreamExt};
use sqlx_core::{connection::Connection, executor::Executor, row::Row};

#[inline]
macro_rules! query {
  ($conn:expr, $query:expr, $cb:expr) => {{
    let rows = sqlx_core::query::query($query).fetch($conn);
    Ok(rows.map($cb).try_collect::<Vec<_>>().await?)
  }};
}

macro_rules! create_sqlx_back_end {
  (
    $(#[$new_doc:meta])+
    $back_end_name:ident,
    $all_tables:expr,
    $clean:expr,
    $conn_ty:ty,
    $create_oapth_tables:expr,
    $insert_migrations:ident($schema:expr)
  ) => {
    /// Wraps functionalities for the `sqlx` crate
    #[derive(Debug)]
    pub struct $back_end_name {
      conn: $conn_ty,
    }

    impl $back_end_name {
      $(#[$new_doc])+
      #[inline]
      pub async fn new(config: &Config) -> crate::Result<Self> {
        let conn = <$conn_ty>::connect(config.url()).await?;
        Ok(Self { conn })
      }
    }

    impl BackEnd for $back_end_name {}

    impl _BackEnd for $back_end_name {
      #[inline]
      fn all_tables<'a>(&'a mut self, schema: &'a str) -> BoxFut<'a, crate::Result<Vec<String>>> {
        Box::pin(async move {
          let buffer = $all_tables(schema)?;
          query!(&mut self.conn, buffer.as_str(), |e| Ok::<_, crate::error::Error>(e?.try_get(0)?))
        })
      }

      #[cfg(feature = "dev-tools")]
      #[inline]
      fn clean<'a>(&'a mut self) -> BoxFut<'a, crate::Result<()>> {
        Box::pin(async move {
          Ok(self.execute(&$clean()?).await?)
        })
      }

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
          let query = _migrations_by_mg_version_query(mg.version(), $schema)?;
          query!(&mut self.conn, query.as_str(), |el| DbMigration::try_from(el.map_err(crate::Error::Sqlx)?))
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
create_sqlx_back_end!(
  /// Creates a new instance from all necessary parameters.
  ///
  /// # Example
  ///
  #[cfg_attr(feature = "_integration-tests", doc = "```rust")]
  #[cfg_attr(not(feature = "_integration-tests"), doc = "```ignore,rust")]
  /// # #[tokio::main] async fn main() -> oapth::Result<()> {
  /// use oapth::{Config, SqlxMssql};
  /// let _ = SqlxMssql::new(&Config::with_url_from_default_var()?).await?;
  /// # Ok(()) }
  SqlxMssql,
  crate::fixed_sql_commands::mssql::_all_tables,
  crate::fixed_sql_commands::mssql::_clean,
  sqlx_core::mssql::MssqlConnection,
  crate::fixed_sql_commands::mssql::_CREATE_MIGRATION_TABLES,
  _insert_migrations(crate::_OAPTH_SCHEMA_PREFIX)
);

#[cfg(feature = "with-sqlx-mysql")]
create_sqlx_back_end!(
  /// Creates a new instance from all necessary parameters.
  ///
  /// # Example
  ///
  #[cfg_attr(feature = "_integration-tests", doc = "```rust")]
  #[cfg_attr(not(feature = "_integration-tests"), doc = "```ignore,rust")]
  /// # #[tokio::main] async fn main() -> oapth::Result<()> {
  /// use oapth::{Config, SqlxMysql};
  /// let _ = SqlxMysql::new(&Config::with_url_from_default_var()?).await?;
  /// # Ok(()) }
  SqlxMysql,
  crate::fixed_sql_commands::mysql::_all_tables,
  crate::fixed_sql_commands::mysql::_clean,
  sqlx_core::mysql::MySqlConnection,
  crate::fixed_sql_commands::mysql::_CREATE_MIGRATION_TABLES,
  _insert_migrations("")
);

#[cfg(feature = "with-sqlx-postgres")]
create_sqlx_back_end!(
  /// Creates a new instance from all necessary parameters.
  ///
  /// # Example
  ///
  #[cfg_attr(feature = "_integration-tests", doc = "```rust")]
  #[cfg_attr(not(feature = "_integration-tests"), doc = "```ignore,rust")]
  /// # #[tokio::main] async fn main() -> oapth::Result<()> {
  /// use oapth::{Config, SqlxPostgres};
  /// let _ = SqlxPostgres::new(&Config::with_url_from_default_var()?).await?;
  /// # Ok(()) }
  SqlxPostgres,
  crate::fixed_sql_commands::postgres::_all_tables,
  crate::fixed_sql_commands::postgres::_clean,
  sqlx_core::postgres::PgConnection,
  crate::fixed_sql_commands::postgres::_CREATE_MIGRATION_TABLES,
  _insert_migrations(crate::_OAPTH_SCHEMA_PREFIX)
);

#[cfg(feature = "with-sqlx-sqlite")]
create_sqlx_back_end!(
  /// Creates a new instance from all necessary parameters.
  ///
  /// # Example
  ///
  #[cfg_attr(feature = "_integration-tests", doc = "```rust")]
  #[cfg_attr(not(feature = "_integration-tests"), doc = "```ignore,rust")]
  /// # #[tokio::main] async fn main() -> oapth::Result<()> {
  /// use oapth::{Config, SqlxSqlite};
  /// let _ = SqlxSqlite::new(&Config::with_url_from_default_var()?).await?;
  /// # Ok(()) }
  SqlxSqlite,
  crate::fixed_sql_commands::sqlite::_all_tables,
  crate::fixed_sql_commands::sqlite::_clean,
  sqlx_core::sqlite::SqliteConnection,
  crate::fixed_sql_commands::sqlite::_CREATE_MIGRATION_TABLES,
  _insert_migrations("")
);
