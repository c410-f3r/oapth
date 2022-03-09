use crate::{
  fixed_sql_commands::{delete_migrations, insert_migrations, migrations_by_mg_version_query},
  Backend, BackendGeneric, Config, DbMigration, MigrationGroup, Migration,
};
use alloc::string::String;
use futures::{StreamExt, TryStreamExt};
use oapth_commons::Database;
use sqlx_core::{connection::Connection, executor::Executor, row::Row};

macro_rules! query {
  ($conn:expr, $query:expr, $cb:expr) => {{
    let rows = sqlx_core::query::query($query).fetch($conn);
    rows.map($cb).try_collect::<Vec<_>>().await
  }};
}

macro_rules! create_sqlx_backend {
  (
    $(#[$new_doc:meta])+
    $backend_name:ident,
    $clean:expr,
    $conn_ty:ty,
    $create_oapth_tables:expr,
    $database:expr,
    $insert_migrations:ident($schema:expr)
    $tables:expr,
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

    impl Backend for $backend_name {}

    #[async_trait::async_trait]
    impl BackendGeneric for $backend_name {
      #[oapth_macros::_dev_tools]
      #[inline]
      async fn clean(&mut self, buffer: &mut String) -> crate::Result<()> {
        $clean(self, buffer).await
      }

      #[inline]
      async fn create_oapth_tables(&mut self) -> crate::Result<()> {
        self.execute($create_oapth_tables).await
      }

      #[inline]
      fn database() -> Database {
        $database
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
        delete_migrations(self, buffer, mg, $schema, version).await
      }

      #[inline]
      async fn execute(&mut self, command: &str) -> crate::Result<()> {
        Ok(self.conn.execute(command).await.map(|_| {})?)
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
        insert_migrations(self, buffer, mg, $schema, migrations).await
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
        use futures::{StreamExt, TryStreamExt};
        migrations_by_mg_version_query(buffer, mg.version(), $schema)?;
        let rslt = query!(&mut self.conn, buffer, |el| DbMigration::try_from(el.map_err(crate::Error::Sqlx)?));
        buffer.clear();
        rslt
      }

      #[inline]
      async fn query_string(&mut self, query: &str) -> crate::Result<Vec<String>> {
        query!(&mut self.conn, query, |e| Ok::<_, crate::error::Error>(e?.try_get(0)?))
      }

      #[inline]
      async fn tables(&mut self, schema: &str) -> crate::Result<Vec<String>> {    
        let buffer = $tables(schema)?;
        query!(&mut self.conn, buffer.as_str(), |e| Ok::<_, crate::error::Error>(e?.try_get(0)?))
      }

      #[inline]
      async fn transaction<I, S>(&mut self, commands: I) -> crate::Result<()>
      where
        I: Iterator<Item = S> + Send,
        S: AsRef<str> + Send + Sync,
      {
        let mut transaction = self.conn.begin().await?;
        for command in commands {
          let _ = transaction.execute(command.as_ref()).await?;
        }
        transaction.commit().await?;
        Ok(())
      }
    }
  };
}

#[oapth_macros::_sqlx_mssql]
create_sqlx_backend!(
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
create_sqlx_backend!(
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
create_sqlx_backend!(
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
create_sqlx_backend!(
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
