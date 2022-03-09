mod schemas;

use crate::{
  fixed_sql_commands::{delete_migrations, insert_migrations, migrations_by_mg_version_query},
  Backend, BackendGeneric, Config, DbMigration, MigrationGroup, Migration
};
use oapth_commons::Database;
use alloc::string::String;
use diesel::{
  connection::{SimpleConnection, TransactionManager},
  sql_query, Connection, RunQueryDsl,
};
use schemas::GenericTable;

macro_rules! create_diesel_backend {
  (
    $(#[$new_doc:meta])+
    $backend_name:ident,
    $clean:path,
    $conn_ty:ty,
    $database:expr,
    $create_oapth_tables:expr,
    $insert_migrations:ident($schema:expr),
    $tables:path,
  ) => {
    /// Wraps functionalities for the `diesel` crate
    #[
      // Diesel types doesn't implement Debug
      allow(missing_debug_implementations)
    ]
    pub struct $backend_name {
      conn: $conn_ty,
    }

    impl $backend_name {
      $(#[$new_doc])+
      #[inline]
      pub async fn new(config: &Config) -> crate::Result<Self> {
        let conn = <$conn_ty>::establish(config.url())?;
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
        Ok(if command.is_empty() {
        }
        else {
          self.conn.batch_execute(command).map(|_| {})?
        })
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
        migrations_by_mg_version_query(buffer, mg.version(), $schema)?;
        let migrations = sql_query(&*buffer).load(&mut self.conn)?;
        buffer.clear();
        Ok(migrations)
      }

      #[inline]
      async fn query_string(&mut self, query: &str) -> crate::Result<Vec<String>> {
        let tables: Vec<GenericTable> = sql_query(query).load(&mut self.conn)?;
        Ok(tables.into_iter().map(|el| el.generic_column).collect())
      }

      #[inline]
      async fn tables(&mut self, schema: &str) -> crate::Result<Vec<String>> {
        let tables: Vec<GenericTable> = sql_query($tables(schema)?.as_str()).load(&mut self.conn)?;
        Ok(tables.into_iter().map(|el| el.generic_column).collect())
      }

      #[inline]
      async fn transaction<I, S>(&mut self, commands: I) -> crate::Result<()>
      where
        I: Iterator<Item = S> + Send,
        S: AsRef<str> + Send + Sync,
      {
        let conn = &mut self.conn;
        <$conn_ty as Connection>::TransactionManager::begin_transaction(conn)?;
        for command in commands {
          conn.batch_execute(command.as_ref())?;
        }
        <$conn_ty as Connection>::TransactionManager::commit_transaction(conn)?;
        Ok(())
      }
    }
  };
}

#[oapth_macros::_diesel_mysql]
create_diesel_backend!(
  /// Creates a new instance from all necessary parameters.
  ///
  /// # Example
  ///
  #[cfg_attr(feature = "_integration-tests", doc = "```rust")]
  #[cfg_attr(not(feature = "_integration-tests"), doc = "```ignore,rust")]
  /// # #[tokio::main] async fn main() -> oapth::Result<()> {
  /// use oapth::{Config, DieselMysql};
  /// let _ = DieselMysql::new(&Config::with_url_from_default_var()?).await?;
  /// # Ok(()) }
  DieselMysql,
  crate::fixed_sql_commands::mysql::clean,
  diesel::mysql::MysqlConnection,
  Database::Mysql,
  crate::fixed_sql_commands::mysql::CREATE_MIGRATION_TABLES,
  _insert_migrations(""),
  crate::fixed_sql_commands::mysql::tables,
);

#[oapth_macros::_diesel_pg]
create_diesel_backend!(
  /// Creates a new instance from all necessary parameters.
  ///
  /// # Example
  ///
  #[cfg_attr(feature = "_integration-tests", doc = "```rust")]
  #[cfg_attr(not(feature = "_integration-tests"), doc = "```ignore,rust")]
  /// # #[tokio::main] async fn main() -> oapth::Result<()> {
  /// use oapth::{Config, DieselPg};
  /// let _ = DieselPg::new(&Config::with_url_from_default_var()?).await?;
  /// # Ok(()) }
  DieselPg,
  crate::fixed_sql_commands::pg::clean,
  diesel::pg::PgConnection,
  Database::Pg,
  crate::fixed_sql_commands::pg::CREATE_MIGRATION_TABLES,
  _insert_migrations(crate::OAPTH_SCHEMA_PREFIX),
  crate::fixed_sql_commands::pg::tables,
);

#[oapth_macros::_diesel_sqlite]
create_diesel_backend!(
  /// Creates a new instance from all necessary parameters.
  ///
  /// # Example
  ///
  #[cfg_attr(feature = "_integration-tests", doc = "```rust")]
  #[cfg_attr(not(feature = "_integration-tests"), doc = "```ignore,rust")]
  /// # #[tokio::main] async fn main() -> oapth::Result<()> {
  /// use oapth::{Config, DieselSqlite};
  /// let _ = DieselSqlite::new(&Config::with_url_from_default_var()?).await?;
  /// # Ok(()) }
  DieselSqlite,
  crate::fixed_sql_commands::sqlite::clean,
  diesel::sqlite::SqliteConnection,
  Database::Sqlite,
  crate::fixed_sql_commands::sqlite::CREATE_MIGRATION_TABLES,
  _insert_migrations(""),
  crate::fixed_sql_commands::sqlite::tables,
);
