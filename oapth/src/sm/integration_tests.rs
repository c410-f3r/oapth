#![allow(dead_code)]

mod backend;
mod db;
mod generic;
mod schema;

use crate::{
  sm::{
    doc_tests::{migration, migration_group},
    Commands, DbMigration, MigrationGroup, SchemaManagement,
  },
  Identifier,
};
use core::fmt::Write;

macro_rules! create_integration_test {
  ($backend:expr, $buffer:expr, $aux:expr, $($fun:path),*) => {{
    $({
      let (_buffer_cmd, _, _buffer_idents) = $buffer;
      let mut commands = crate::sm::Commands::with_database($backend);
      commands.clear((_buffer_cmd, _buffer_idents)).await.unwrap();
      $fun($buffer, &mut commands, $aux).await;
    })*
  }};
}

macro_rules! _create_integration_test_backend {
  ($backend_ty:ident) => {{
    let c = crate::sm::Config::with_url_from_default_var().unwrap();
    crate::database::$backend_ty::new(&c).await.unwrap()
  }};
}

macro_rules! create_integration_tests {
  (
    $fn_name:ident,
    sqlx_mysql: $($sqlx_mysql:path),*;
    sqlx_pg: $($sqlx_pg:path),*;
    sqlx_sqlite: $($sqlx_sqlite:path),*;
    tiberius: $($tiberius:path),*;
  ) => {
    pub(crate) async fn $fn_name() {
      let mut _buffer_cmd = String::new();
      let mut _buffer_db_migrations = Vec::<DbMigration>::new();
      let mut _buffer_idents = Vec::<Identifier>::new();

      #[cfg(feature = "sqlx-mysql")]
      create_integration_test!(
        _create_integration_test_backend!(SqlxMysql),
        (&mut _buffer_cmd, &mut _buffer_db_migrations, &mut _buffer_idents),
        _generic_schema(),
        $($sqlx_mysql),*
      );

      #[cfg(feature = "sqlx-postgres")]
      create_integration_test!(
        _create_integration_test_backend!(SqlxPostgres),
        (&mut _buffer_cmd, &mut _buffer_db_migrations, &mut _buffer_idents),
        _pg_schema(),
        $($sqlx_pg),*
      );

      #[cfg(feature = "sqlx-sqlite")]
      create_integration_test!(
        _create_integration_test_backend!(SqlxSqlite),
        (&mut _buffer_cmd, &mut _buffer_db_migrations, &mut _buffer_idents),
        _generic_schema(),
        $($sqlx_sqlite),*
      );

      #[cfg(feature = "tiberius")]
      create_integration_test!(
        {
          use tokio_util::compat::TokioAsyncWriteCompatExt;
          let c = crate::sm::Config::with_url_from_default_var().unwrap();
          let tcp = tokio::net::TcpStream::connect(c.full_host().unwrap()).await.unwrap();
          crate::database::Tiberius::new(&c, tcp.compat_write()).await.unwrap()
        },
        (&mut _buffer_cmd, &mut _buffer_db_migrations, &mut _buffer_idents),
        _mssql_schema(),
        $($tiberius),*
      );
    }
  };
}

macro_rules! create_all_integration_tests {
  (
    sqlx_mysql: $($sqlx_mysql:path),*;
    sqlx_pg: $($sqlx_pg:path),*;
    sqlx_sqlite: $($sqlx_sqlite:path),*;
    tiberius: $($tiberius:path),*;

    mssql: $($mssql:path),*;
    mysql: $($mysql:path),*;
    postgres: $($postgres:path),*;
    sqlite: $($sqlite:path),*;

    generic: $($fun:path),*;

    with_schema: $($with_schema:path),*;
    without_schema: $($without_schema:path),*;
  ) => {
    create_integration_tests!(
      integration_tests_backend,
      sqlx_mysql: $($sqlx_mysql),*;
      sqlx_pg: $($sqlx_pg),*;
      sqlx_sqlite: $($sqlx_sqlite),*;
      tiberius: $($tiberius),*;
    );

    create_integration_tests!(
      integration_tests_db,
      sqlx_mysql: $($mysql),*;
      sqlx_pg: $($postgres),*;
      sqlx_sqlite: $($sqlite),*;
      tiberius: $($mssql),*;
    );

    create_integration_tests!(
      integration_tests_generic,
      sqlx_mysql: $($fun),*;
      sqlx_pg: $($fun),*;
      sqlx_sqlite: $($fun),*;
      tiberius: $($fun),*;
    );

    create_integration_tests!(
      integration_tests_schema,
      sqlx_mysql: $($without_schema),*;
      sqlx_pg: $($with_schema),*;
      sqlx_sqlite: $($without_schema),*;
      tiberius: $($with_schema),*;
    );

    #[tokio::test]
    async fn integration_tests() {
      integration_tests_backend().await;
      integration_tests_db().await;
      integration_tests_generic().await;
      integration_tests_schema().await;
    }
  };
}

create_all_integration_tests!(
  // Back end

  sqlx_mysql:
    backend::_backend_has_migration_with_utc_time
    ;
  sqlx_pg: ;
  sqlx_sqlite:
    backend::_backend_has_migration_with_utc_time;
  tiberius:
    backend::_backend_has_migration_with_utc_time;

  // Database

  mssql:
    db::mssql::_clean_drops_all_objs;
  mysql:
    db::mysql::_clean_drops_all_objs;
  postgres:
    db::postgres::_clean_drops_all_objs;
  sqlite:
    db::sqlite::_clean_drops_all_objs;

  // Generic

  generic:
    generic::all_tables_returns_the_number_of_tables_of_the_default_schema,
    generic::rollback_works;

  // Schema

  with_schema:
    schema::with_schema::all_tables_returns_the_number_of_tables_of_oapth_schema,
    schema::with_schema::migrate_works;
  without_schema:
    schema::without_schema::_migrate_works;
);

#[derive(Clone, Copy)]
pub(crate) struct AuxTestParams {
  pub(crate) default_schema: &'static str,
  pub(crate) oapth_schema: &'static str,
  pub(crate) schema_regulator: usize,
}

pub(crate) async fn create_foo_table<D>(
  buffer_cmd: &mut String,
  c: &mut Commands<D>,
  schema_prefix: &str,
) where
  D: SchemaManagement,
{
  buffer_cmd.write_fmt(format_args!("CREATE TABLE {}foo(id INT)", schema_prefix)).unwrap();
  c.database.execute(buffer_cmd).await.unwrap();
  buffer_cmd.clear();
}

#[inline]
pub(crate) fn _generic_schema() -> AuxTestParams {
  AuxTestParams { default_schema: "", oapth_schema: "", schema_regulator: 2 }
}

#[inline]
pub(crate) async fn _migrate_doc_test<D>(
  (buffer_cmd, buffer_db_migrations, _): (&mut String, &mut Vec<DbMigration>, &mut Vec<Identifier>),
  c: &mut Commands<D>,
) -> MigrationGroup<&'static str>
where
  D: SchemaManagement,
{
  let mg = migration_group();
  c.migrate((buffer_cmd, buffer_db_migrations), &mg, [migration()].iter()).await.unwrap();
  mg
}

#[inline]
pub(crate) fn _mssql_schema() -> AuxTestParams {
  AuxTestParams { default_schema: "dbo", oapth_schema: "_oapth", schema_regulator: 0 }
}

#[inline]
pub(crate) fn _pg_schema() -> AuxTestParams {
  AuxTestParams { default_schema: "public", oapth_schema: "_oapth", schema_regulator: 0 }
}
