use crate::{
  fixed_sql_commands::{delete_migrations, insert_migrations, migrations_by_mg_version_query},
  BackEnd, BackEndGeneric, BoxFut, Config, DbMigration, MigrationGroup, Migration, Database
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
    $clean:expr,
    $conn_ty:ty,
    $create_oapth_tables:expr,
    $database:expr,
    $insert_migrations:ident($schema:expr)
    $tables:expr,
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

    impl BackEndGeneric for $back_end_name {
      #[oapth_macros::dev_tools_]
      #[inline]
      fn clean<'a>(&'a mut self) -> BoxFut<'a, crate::Result<()>> {
        Box::pin(async move {
          let clean = &$clean(self).await?;
          Ok(self.execute(&clean).await?)
        })
      }

      #[inline]
      fn create_oapth_tables<'a>(&'a mut self) -> BoxFut<'a, crate::Result<()>> {
        self.execute($create_oapth_tables)
      }

      #[inline]
      fn database() -> Database {
        $database
      }

      #[inline]
      fn delete_migrations<'a>(
        &'a mut self,
        version: i32,
        mg: &'a MigrationGroup,
      ) -> BoxFut<'a, crate::Result<()>> {
        Box::pin(async move { Ok(delete_migrations(self, mg, $schema, version).await?) })
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
        Box::pin(insert_migrations(self, mg, $schema, migrations))
      }

      #[inline]
      fn migrations<'a>(&'a mut self, mg: &'a MigrationGroup,) -> BoxFut<'a, crate::Result<Vec<DbMigration>>> {
        use futures::{StreamExt, TryStreamExt};
        Box::pin(async move {
          let query = migrations_by_mg_version_query(mg.version(), $schema)?;
          query!(&mut self.conn, query.as_str(), |el| DbMigration::try_from(el.map_err(crate::Error::Sqlx)?))
        })
      }

      #[inline]
      fn query_string<'a>(&'a mut self, query: &'a str) -> BoxFut<'a, crate::Result<Vec<String>>> {
        Box::pin(async move {
          query!(&mut self.conn, query, |e| Ok::<_, crate::error::Error>(e?.try_get(0)?))
        })
      }

      #[inline]
      fn tables<'a>(&'a mut self, schema: &'a str) -> BoxFut<'a, crate::Result<Vec<String>>> {
        Box::pin(async move {
          let buffer = $tables(schema)?;
          query!(&mut self.conn, buffer.as_str(), |e| Ok::<_, crate::error::Error>(e?.try_get(0)?))
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

#[oapth_macros::sqlx_mssql_]
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
  crate::fixed_sql_commands::mssql::clean,
  sqlx_core::mssql::MssqlConnection,
  crate::fixed_sql_commands::mssql::CREATE_MIGRATION_TABLES,
  Database::Mssql,
  _insert_migrations(crate::OAPTH_SCHEMA_PREFIX)
  crate::fixed_sql_commands::mssql::tables,
);

#[oapth_macros::sqlx_mysql_]
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
  crate::fixed_sql_commands::mysql::clean,
  sqlx_core::mysql::MySqlConnection,
  crate::fixed_sql_commands::mysql::CREATE_MIGRATION_TABLES,
  Database::Mysql,
  _insert_migrations("")
  crate::fixed_sql_commands::mysql::tables,
);

#[oapth_macros::sqlx_pg_]
create_sqlx_back_end!(
  /// Creates a new instance from all necessary parameters.
  ///
  /// # Example
  ///
  #[cfg_attr(feature = "_integration-tests", doc = "```rust")]
  #[cfg_attr(not(feature = "_integration-tests"), doc = "```ignore,rust")]
  /// # #[tokio::main] async fn main() -> oapth::Result<()> {
  /// use oapth::{Config, SqlxPg};
  /// let _ = SqlxPg::new(&Config::with_url_from_default_var()?).await?;
  /// # Ok(()) }
  SqlxPg,
  crate::fixed_sql_commands::pg::clean,
  sqlx_core::postgres::PgConnection,
  crate::fixed_sql_commands::pg::CREATE_MIGRATION_TABLES,
  Database::Pg,
  _insert_migrations(crate::OAPTH_SCHEMA_PREFIX)
  crate::fixed_sql_commands::pg::tables,
);

#[oapth_macros::sqlx_sqlite_]
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
  crate::fixed_sql_commands::sqlite::clean,
  sqlx_core::sqlite::SqliteConnection,
  crate::fixed_sql_commands::sqlite::CREATE_MIGRATION_TABLES,
  Database::Sqlite,
  _insert_migrations("")
  crate::fixed_sql_commands::sqlite::tables,
);
