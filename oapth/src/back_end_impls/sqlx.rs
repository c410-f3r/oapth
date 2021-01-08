use crate::{
  fixed_sql_commands::{delete_migrations, insert_migrations, migrations_by_mg_version_query},
  BackEnd, BackEndGeneric, BoxFut, Config, DbMigration, MigrationGroupRef, MigrationRef,
};
use alloc::string::String;
use core::convert::TryFrom;
use futures::{StreamExt, TryStreamExt};
use oapth_commons::Database;
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
      #[oapth_macros::_dev_tools]
      #[inline]
      fn clean<'a, 'ret>(&'a mut self) -> BoxFut<'ret, crate::Result<()>>
      where
        'a: 'ret,
        Self: 'ret,
      {
        Box::pin(async move {
          Ok($clean(self).await?)
        })
      }

      #[inline]
      fn create_oapth_tables<'a, 'ret>(&'a mut self) -> BoxFut<'ret, crate::Result<()>>
      where
        'a: 'ret,
        Self: 'ret,
      {
        self.execute($create_oapth_tables)
      }

      #[inline]
      fn database() -> Database {
        $database
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
        Box::pin(async move { Ok(delete_migrations(self, mg, $schema, version).await?) })
      }

      #[inline]
      fn execute<'a, 'b, 'ret>(&'a mut self, command: &'b str) -> BoxFut<'ret, crate::Result<()>>
      where
        'a: 'ret,
        'b: 'ret,
        Self: 'ret,
      {
        Box::pin(async move { Ok(self.conn.execute(command).await.map(|_| {})?) })
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
        Box::pin(insert_migrations(self, mg, $schema, migrations))
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
        use futures::{StreamExt, TryStreamExt};
        Box::pin(async move {
          let query = migrations_by_mg_version_query(mg.version(), $schema)?;
          query!(&mut self.conn, query.as_str(), |el| DbMigration::try_from(el.map_err(crate::Error::Sqlx)?))
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
          query!(&mut self.conn, query, |e| Ok::<_, crate::error::Error>(e?.try_get(0)?))
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
          let buffer = $tables(schema)?;
          query!(&mut self.conn, buffer.as_str(), |e| Ok::<_, crate::error::Error>(e?.try_get(0)?))
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
          let mut transaction = self.conn.begin().await?;
          for command in commands {
            let _ = transaction.execute(command.as_ref()).await?;
          }
          transaction.commit().await?;
          Ok(())
        })
      }
    }
  };
}

#[oapth_macros::_sqlx_mssql]
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

#[oapth_macros::_sqlx_mysql]
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

#[oapth_macros::_sqlx_pg]
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

#[oapth_macros::_sqlx_sqlite]
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
