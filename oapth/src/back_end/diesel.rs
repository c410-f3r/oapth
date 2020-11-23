mod schemas;

use crate::{
  fixed_sql_commands::{_delete_migrations, _insert_migrations, _migrations_by_mg_version_query},
  BackEnd, BoxFut, Config, DbMigration, Migration, MigrationGroup, _BackEnd,
};
use alloc::string::String;
use diesel::{
  connection::{SimpleConnection, TransactionManager},
  sql_query, Connection, RunQueryDsl,
};
use schemas::AllTables;

macro_rules! create_diesel_back_end {
  (
    $(#[$new_doc:meta])+
    $back_end_name:ident,
    $all_tables:path,
    $clean:path,
    $conn_ty:ty,
    $create_oapth_tables:expr,
    $insert_migrations:ident($schema:expr),
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

    impl _BackEnd for $back_end_name {
      #[inline]
      fn all_tables<'a>(&'a mut self, schema: &'a str) -> BoxFut<'a, crate::Result<Vec<String>>> {
        Box::pin(async move {
          let a: Vec<AllTables> = sql_query($all_tables(schema)?.as_str()).load(&self.conn)?;
          Ok(a.into_iter().map(|el| el.table_name).collect())
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
      fn execute<'b>(&'b mut self, command: &'b str) -> BoxFut<'b, crate::Result<()>> {
        Box::pin(async move { Ok(self.conn.batch_execute(command).map(|_| {})?) })
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
        Box::pin(async move {
          let query = _migrations_by_mg_version_query(mg.version(), $schema)?;
          let migrations = sql_query(query.as_str()).load(&self.conn)?;
          Ok(migrations)
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

#[cfg(feature = "with-diesel-mysql")]
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
  crate::fixed_sql_commands::mysql::_all_tables,
  crate::fixed_sql_commands::mysql::_clean,
  diesel::mysql::MysqlConnection,
  crate::fixed_sql_commands::mysql::_CREATE_MIGRATION_TABLES,
  _insert_migrations(""),
);

#[cfg(feature = "with-diesel-postgres")]
create_diesel_back_end!(
  /// Creates a new instance from all necessary parameters.
  ///
  /// # Example
  ///
  #[cfg_attr(feature = "_integration-tests", doc = "```rust")]
  #[cfg_attr(not(feature = "_integration-tests"), doc = "```ignore,rust")]
  /// # #[tokio::main] async fn main() -> oapth::Result<()> {
  /// use oapth::{Config, DieselPostgres};
  /// let _ = DieselPostgres::new(&Config::with_url_from_default_var()?).await?;
  /// # Ok(()) }
  DieselPostgres,
  crate::fixed_sql_commands::postgres::_all_tables,
  crate::fixed_sql_commands::postgres::_clean,
  diesel::pg::PgConnection,
  crate::fixed_sql_commands::postgres::_CREATE_MIGRATION_TABLES,
  _insert_migrations(crate::_OAPTH_SCHEMA_PREFIX),
);

#[cfg(feature = "with-diesel-sqlite")]
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
  crate::fixed_sql_commands::sqlite::_all_tables,
  crate::fixed_sql_commands::sqlite::_clean,
  diesel::sqlite::SqliteConnection,
  crate::fixed_sql_commands::sqlite::_CREATE_MIGRATION_TABLES,
  _insert_migrations(""),
);
