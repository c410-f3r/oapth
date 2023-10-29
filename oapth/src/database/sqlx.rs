use crate::{database::Database, Config, DatabaseTy};
#[cfg(feature = "sm")]
use alloc::{string::String, vec::Vec};
use futures::TryStreamExt;
use sqlx_core::{connection::Connection, executor::Executor, pool::PoolConnection, query::query};

macro_rules! create_sqlx_backend {
  (
    $(#[$new_doc:meta])+
    $backend_name:ident,
    $clear:expr,
    $conn_ty:ty,
    $create_oapth_tables:expr,
    $db:ty,
    $db_ty:expr,
    ($schema:expr, $mysql_schema:expr),
    $row:ty,
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

    impl Database for $backend_name {
      const TY: DatabaseTy = $db_ty;

      type Row = $row;

      #[inline]
      async fn execute(&mut self, cmd: &str) -> crate::Result<()> {
        Ok(self.conn.execute(cmd).await.map(|_| {})?)
      }

      #[inline]
      async fn row(&mut self, cmd: &str) -> crate::Result<Self::Row> {
        Ok(query(cmd).fetch_one(&mut self.conn).await?)
      }

      #[inline]
      async fn rows<E>(
        &mut self,
        cmd: &str,
        mut cb: impl FnMut(Self::Row) -> Result<(), E>,
      ) -> Result<(), E>
      where
        E: From<crate::Error>
      {
        let mut stream = query(cmd).fetch(&mut self.conn);
        while let Some(row) = stream.try_next().await.map_err(From::from)? {
          cb(row)?;
        }
        Ok(())
      }

      #[inline]
      async fn transaction(&mut self, cmd: &str) -> crate::Result<()> {
        let mut transaction = self.conn.begin().await?;
        let _ = transaction.execute(cmd).await?;
        transaction.commit().await?;
        Ok(())
      }
    }

    impl Database for PoolConnection<$db> {
      const TY: DatabaseTy = $db_ty;

      type Row = $row;

      #[inline]
      async fn execute(&mut self, cmd: &str) -> crate::Result<()> {
        Ok((**self).execute(cmd).await.map(|_| {})?)
      }

      #[inline]
      async fn row(&mut self, cmd: &str) -> crate::Result<Self::Row> {
        Ok(query(cmd).fetch_one(&mut **self).await?)
      }

      #[inline]
      async fn rows<E>(
        &mut self,
        cmd: &str,
        mut cb: impl FnMut(Self::Row) -> Result<(), E>,
      ) -> Result<(), E>
      where
        E: From<crate::Error>
      {
        let mut stream = query(cmd).fetch(&mut **self);
        while let Some(row) = stream.try_next().await.map_err(From::from)? {
          cb(row)?;
        }
        Ok(())
      }

      #[inline]
      async fn transaction(&mut self, cmd: &str) -> crate::Result<()> {
        let mut transaction = self.begin().await?;
        let _ = transaction.execute(cmd).await?;
        transaction.commit().await?;
        Ok(())
      }
    }

    #[cfg(feature = "sm")]
    impl crate::sm::SchemaManagement for $backend_name {
      #[inline]
      async fn clear(
        &mut self,
        buffer: (&mut String, &mut Vec<crate::Identifier>),
      ) -> crate::Result<()> {
        $clear(buffer.into(), self).await
      }

      #[inline]
      async fn create_oapth_tables(&mut self) -> crate::Result<()> {
        self.execute($create_oapth_tables).await
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
          $schema,
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
          $schema,
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
        crate::sm::fixed_sql_commands::_migrations_by_mg_version_query::<crate::Error, Self>(
          buffer_cmd,
          self,
          mg.version(),
          results,
          $schema,
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
        let actual_schema = if $mysql_schema.is_empty() {
          schema
        } else {
          $mysql_schema
        };
        $tables(buffer_cmd, self, results, actual_schema).await
      }
    }
  };
}

#[cfg(feature = "sqlx-mysql")]
create_sqlx_backend!(
  /// Creates a new instance from all necessary parameters.
  ///
  /// # Example
  ///
  #[cfg_attr(feature = "_integration-tests", doc = "```rust")]
  #[cfg_attr(not(feature = "_integration-tests"), doc = "```ignore,rust")]
  /// # #[tokio::main] async fn main() -> oapth::Result<()> {
  /// use oapth::{Config, database::SqlxMysql};
  /// let _ = SqlxMysql::new(&Config::with_url_from_default_var()?).await?;
  /// # Ok(()) }
  SqlxMysql,
  crate::sm::fixed_sql_commands::mysql::_clear,
  sqlx_mysql::MySqlConnection,
  crate::sm::fixed_sql_commands::mysql::_CREATE_MIGRATION_TABLES,
  sqlx_mysql::MySql,
  DatabaseTy::MySql,
  ("", crate::sm::_OAPTH),
  sqlx_mysql::MySqlRow,
  crate::sm::fixed_sql_commands::mysql::_table_names,
);

#[cfg(feature = "sqlx-postgres")]
create_sqlx_backend!(
  /// Creates a new instance from all necessary parameters.
  ///
  /// # Example
  ///
  #[cfg_attr(feature = "_integration-tests", doc = "```rust")]
  #[cfg_attr(not(feature = "_integration-tests"), doc = "```ignore,rust")]
  /// # #[tokio::main] async fn main() -> oapth::Result<()> {
  /// use oapth::{database::SqlxPostgres, Config};
  /// let _ = SqlxPostgres::new(&Config::with_url_from_default_var()?).await?;
  /// # Ok(()) }
  SqlxPostgres,
  crate::sm::fixed_sql_commands::postgres::_clear,
  sqlx_postgres::PgConnection,
  crate::sm::fixed_sql_commands::postgres::_CREATE_MIGRATION_TABLES,
  sqlx_postgres::Postgres,
  DatabaseTy::Postgres,
  (crate::sm::_OAPTH_SCHEMA_PREFIX, ""),
  sqlx_postgres::PgRow,
  crate::sm::fixed_sql_commands::postgres::_table_names,
);

#[cfg(feature = "sqlx-sqlite")]
create_sqlx_backend!(
  /// Creates a new instance from all necessary parameters.
  ///
  /// # Example
  ///
  #[cfg_attr(feature = "_integration-tests", doc = "```rust")]
  #[cfg_attr(not(feature = "_integration-tests"), doc = "```ignore,rust")]
  /// # #[tokio::main] async fn main() -> oapth::Result<()> {
  /// use oapth::{Config, database::SqlxSqlite};
  /// let _ = SqlxSqlite::new(&Config::with_url_from_default_var()?).await?;
  /// # Ok(()) }
  SqlxSqlite,
  crate::sm::fixed_sql_commands::sqlite::_clear,
  sqlx_sqlite::SqliteConnection,
  crate::sm::fixed_sql_commands::sqlite::_CREATE_MIGRATION_TABLES,
  sqlx_sqlite::Sqlite,
  DatabaseTy::Sqlite,
  ("", ""),
  sqlx_sqlite::SqliteRow,
  crate::sm::fixed_sql_commands::sqlite::_table_names,
);
