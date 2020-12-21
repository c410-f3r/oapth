mod schemas;

use crate::{
  fixed_sql_commands::{delete_migrations, insert_migrations, migrations_by_mg_version_query},
  BackEnd, BackEndGeneric, BoxFut, Config, DbMigration,  MigrationGroup, Database, Migration
};
use alloc::string::String;
use diesel::{
  connection::{SimpleConnection, TransactionManager},
  sql_query, Connection, RunQueryDsl,
};
use schemas::GenericTable;

macro_rules! create_diesel_back_end {
  (
    $(#[$new_doc:meta])+
    $back_end_name:ident,
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
    pub struct $back_end_name {
      conn: $conn_ty,
    }

    impl $back_end_name {
      $(#[$new_doc])+
      #[inline]
      pub async fn new(config: &Config) -> crate::Result<Self> {
        let conn = <$conn_ty>::establish(config.url())?;
        Ok(Self { conn })
      }
    }

    impl BackEnd for $back_end_name {
    }

    impl BackEndGeneric for $back_end_name {
      #[oapth_macros::dev_tools_]
      #[inline]
      fn clean<'a>(&'a mut self) -> BoxFut<'a, crate::Result<()>> {
        Box::pin(async move {
          Ok($clean(self).await?)
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
      fn execute<'b>(&'b mut self, command: &'b str) -> BoxFut<'b, crate::Result<()>> {
        Box::pin(async move {
          Ok(if command.is_empty() {
            ()
          }
          else {
            self.conn.batch_execute(command).map(|_| {})?
          })
        })
      }

      #[inline]
      fn insert_migrations<'a, 'b, 'c, 'ret, I>(
        &'a mut self,
        migrations: I,
        mg: &'b MigrationGroup,
      ) -> BoxFut<'ret, crate::Result<()>>
      where
        'a: 'ret,
        'b: 'ret,
        'c: 'ret,
        I: Clone + Iterator<Item = &'c Migration> + 'ret,
        Self: 'ret
      {
        Box::pin(insert_migrations(self, mg, $schema, migrations))
      }

      #[inline]
      fn migrations<'a>(&'a mut self, mg: &'a MigrationGroup,) -> BoxFut<'a, crate::Result<Vec<DbMigration>>> {
        Box::pin(async move {
          let query = migrations_by_mg_version_query(mg.version(), $schema)?;
          let migrations = sql_query(query.as_str()).load(&self.conn)?;
          Ok(migrations)
        })
      }

      #[inline]
      fn query_string<'a>(&'a mut self, query: &'a str) -> BoxFut<'a, crate::Result<Vec<String>>> {
        Box::pin(async move {
          let tables: Vec<GenericTable> = sql_query(query).load(&self.conn)?;
          Ok(tables.into_iter().map(|el| el.generic_column).collect())
        })
      }

      #[inline]
      fn tables<'a>(&'a mut self, schema: &'a str) -> BoxFut<'a, crate::Result<Vec<String>>> {
        Box::pin(async move {
          let tables: Vec<GenericTable> = sql_query($tables(schema)?.as_str()).load(&self.conn)?;
          Ok(tables.into_iter().map(|el| el.generic_column).collect())
        })
      }

      #[inline]
      fn transaction<'a, I, S>(&'a mut self, commands: I) -> BoxFut<'a, crate::Result<()>>
      where
        I: Iterator<Item = S> + 'a,
        S: AsRef<str>,
      {
        Box::pin(async move {
          let conn = &mut self.conn;
          let transaction_manager = conn.transaction_manager();
          transaction_manager.begin_transaction(conn)?;
          for command in commands {
            conn.batch_execute(command.as_ref())?;
          }
          transaction_manager.commit_transaction(conn)?;
          Ok(())
        })
      }
    }
  };
}

#[oapth_macros::diesel_mysql_]
create_diesel_back_end!(
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

#[oapth_macros::diesel_pg_]
create_diesel_back_end!(
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

#[oapth_macros::diesel_sqlite_]
create_diesel_back_end!(
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
