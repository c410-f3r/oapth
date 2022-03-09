mod backend;
mod db;
mod generic;
mod schema;

use crate::{
  doc_tests::{migration, migration_group},
  Backend, Commands, MigrationGroupRef,
};
use core::fmt::Write;

macro_rules! create_integration_test {
  ($backend:expr, $_buffer:expr, $aux:expr, $($fun:path),*) => {{
    $(
      let mut commands = crate::Commands::with_backend($backend);
      commands.clean(&mut $_buffer).await.unwrap();
      $fun(&mut $_buffer, &mut commands, $aux).await;
    )*
  }};
}

macro_rules! _create_integration_test_backend {
  ($backend_ty:ident) => {{
    let c = crate::Config::with_url_from_default_var().unwrap();
    crate::$backend_ty::new(&c).await.unwrap()
  }};
}

macro_rules! create_integration_tests {
  (
    $fn_name:ident,
    diesel_mysql: $($diesel_mysql:path),*;
    diesel_pg: $($diesel_pg:path),*;
    diesel_sqlite: $($diesel_sqlite:path),*;
    mysql_async: $($mysql_async:path),*;
    rusqlite: $($rusqlite:path),*;
    sqlx_mssql: $($sqlx_mssql:path),*;
    sqlx_mysql: $($sqlx_mysql:path),*;
    sqlx_pg: $($sqlx_pg:path),*;
    sqlx_sqlite: $($sqlx_sqlite:path),*;
    tiberius: $($tiberius:path),*;
    tokio_postgres: $($tokio_postgres:path),*;
  ) => {
    pub(crate) async fn $fn_name() {
      let mut _buffer = String::new();

      oapth_macros::_diesel_mysql_! {
        create_integration_test!(
          _create_integration_test_backend!(DieselMysql),
          _buffer,
          _generic_schema(),
          $($diesel_mysql),*
        );
      }
      oapth_macros::_diesel_pg_! {
        create_integration_test!(
          _create_integration_test_backend!(DieselPg),
          _buffer,
          _pg_schema(),
          $($diesel_pg),*
        );
      }

      oapth_macros::_diesel_sqlite_! {
        create_integration_test!(
          _create_integration_test_backend!(DieselSqlite),
          _buffer,
          _generic_schema(),
          $($diesel_sqlite),*
        );
      }

      oapth_macros::_mysql_async_! {
        create_integration_test!(
          _create_integration_test_backend!(MysqlAsync),
          _buffer,
          _generic_schema(),
          $($mysql_async),*
        );
      }

      oapth_macros::_rusqlite_! {
        create_integration_test!(
          _create_integration_test_backend!(Rusqlite),
          _buffer,
          _generic_schema(),
          $($rusqlite),*
        );
      }

      oapth_macros::_sqlx_mssql_! {
        create_integration_test!(
          _create_integration_test_backend!(SqlxMssql),
          _buffer,
          _mssql_schema(),
          $($sqlx_mssql),*
        );
      }

      oapth_macros::_sqlx_mysql_! {
        create_integration_test!(
          _create_integration_test_backend!(SqlxMysql),
          _buffer,
          _generic_schema(),
          $($sqlx_mysql),*
        );
      }

      oapth_macros::_sqlx_pg_! {
        create_integration_test!(
          _create_integration_test_backend!(SqlxPg),
          _buffer,
          _pg_schema(),
          $($sqlx_pg),*
        );
      }

      oapth_macros::_sqlx_sqlite_! {
        create_integration_test!(
          _create_integration_test_backend!(SqlxSqlite),
          _buffer,
          _generic_schema(),
          $($sqlx_sqlite),*
        );
      }

      oapth_macros::_tiberius_! {
        create_integration_test!(
          {
            use tokio_util::compat::TokioAsyncWriteCompatExt;
            let c = crate::Config::with_url_from_default_var().unwrap();
            let tcp = tokio::net::TcpStream::connect(c.full_host().unwrap()).await.unwrap();
            crate::Tiberius::new(&c, tcp.compat_write()).await.unwrap()
          },
          _buffer,
          _mssql_schema(),
          $($tiberius),*
        );
      }

      oapth_macros::_tokio_postgres_! {
        create_integration_test!(
          _create_integration_test_backend!(TokioPostgres),
          _buffer,
          _pg_schema(),
          $($tokio_postgres),*
        );
      }
    }
  };
}

macro_rules! create_all_integration_tests {
  (
    diesel_mysql: $($diesel_mysql:path),*;
    diesel_pg: $($diesel_pg:path),*;
    diesel_sqlite: $($diesel_sqlite:path),*;
    mysql_async: $($mysql_async:path),*;
    rusqlite: $($rusqlite:path),*;
    sqlx_mssql: $($sqlx_mssql:path),*;
    sqlx_mysql: $($sqlx_mysql:path),*;
    sqlx_pg: $($sqlx_pg:path),*;
    sqlx_sqlite: $($sqlx_sqlite:path),*;
    tiberius: $($tiberius:path),*;
    tokio_postgres: $($tokio_postgres:path),*;

    mssql: $($mssql:path),*;
    mysql: $($mysql:path),*;
    pg: $($pg:path),*;
    sqlite: $($sqlite:path),*;

    generic: $($fun:path),*;

    with_schema: $($with_schema:path),*;
    without_schema: $($without_schema:path),*;
  ) => {
    create_integration_tests!(
      integration_tests_backend,
      diesel_mysql: $($diesel_mysql),*;
      diesel_pg: $($diesel_pg),*;
      diesel_sqlite: $($diesel_sqlite),*;
      mysql_async: $($mysql_async),*;
      rusqlite: $($rusqlite),*;
      sqlx_mssql: $($sqlx_mssql),*;
      sqlx_mysql: $($sqlx_mysql),*;
      sqlx_pg: $($sqlx_pg),*;
      sqlx_sqlite: $($sqlx_sqlite),*;
      tiberius: $($tiberius),*;
      tokio_postgres: $($tokio_postgres),*;
    );

    create_integration_tests!(
      integration_tests_db,
      diesel_mysql: $($mysql),*;
      diesel_pg: $($pg),*;
      diesel_sqlite: $($sqlite),*;
      mysql_async: $($mysql),*;
      rusqlite: $($sqlite),*;
      sqlx_mssql: $($mssql),*;
      sqlx_mysql: $($mysql),*;
      sqlx_pg: $($pg),*;
      sqlx_sqlite: $($sqlite),*;
      tiberius: $($mssql),*;
      tokio_postgres: $($pg),*;
    );

    create_integration_tests!(
      integration_tests_generic,
      diesel_mysql: $($fun),*;
      diesel_pg: $($fun),*;
      diesel_sqlite: $($fun),*;
      mysql_async: $($fun),*;
      rusqlite: $($fun),*;
      sqlx_mssql: $($fun),*;
      sqlx_mysql: $($fun),*;
      sqlx_pg: $($fun),*;
      sqlx_sqlite: $($fun),*;
      tiberius: $($fun),*;
      tokio_postgres: $($fun),*;
    );

    create_integration_tests!(
      integration_tests_schema,
      diesel_mysql: $($without_schema),*;
      diesel_pg: $($with_schema),*;
      diesel_sqlite: $($without_schema),*;
      mysql_async: $($without_schema),*;
      rusqlite: $($without_schema),*;
      sqlx_mssql: $($with_schema),*;
      sqlx_mysql: $($without_schema),*;
      sqlx_pg: $($with_schema),*;
      sqlx_sqlite: $($without_schema),*;
      tiberius: $($with_schema),*;
      tokio_postgres: $($with_schema),*;
    );

    #[tokio::test]
    async fn integration_tests() {
      let _ = env_logger::builder().is_test(true).try_init();

      integration_tests_backend().await;
      integration_tests_db().await;
      integration_tests_generic().await;
      integration_tests_schema().await;
    }
  };
}

create_all_integration_tests!(
  // Back end

  diesel_mysql:
    backend::_backend_has_migration_with_utc_time;
  diesel_pg:
    backend::_backend_has_migration_with_utc_time;
  diesel_sqlite:
    backend::_backend_has_migration_with_utc_time;
  mysql_async:
    backend::_backend_has_migration_with_utc_time;
  rusqlite:
    backend::_backend_has_migration_with_utc_time;
  sqlx_mssql:
    backend::_backend_has_migration_with_utc_time;
  sqlx_mysql:
    backend::_backend_has_migration_with_utc_time;
  sqlx_pg: ;
  sqlx_sqlite:
    backend::_backend_has_migration_with_utc_time;
  tiberius:
    backend::_backend_has_migration_with_utc_time;
  tokio_postgres: ;

  // Database

  mssql:
    db::mssql::clean_drops_all_objs;
  mysql:
    db::mysql::clean_drops_all_objs;
  pg:
    db::pg::clean_drops_all_objs;
  sqlite:
    db::sqlite::clean_drops_all_objs;

  // Generic

  generic:
    generic::all_tables_returns_the_number_of_tables_of_the_default_schema,
    generic::rollback_works;

  // Schema

  with_schema:
    schema::with_schema::all_tables_returns_the_number_of_tables_of_oapth_schema,
    schema::with_schema::migrate_works;
  without_schema:
    schema::without_schema::migrate_works;
);

#[derive(Clone, Copy)]
pub(crate) struct AuxTestParams {
  pub(crate) default_schema: &'static str,
  pub(crate) oapth_schema: &'static str,
  pub(crate) schema_regulator: usize,
}

pub(crate) async fn create_foo_table<B>(c: &mut Commands<B>, schema_prefix: &str)
where
  B: Backend,
{
  let mut _buffer = String::new();
  _buffer.write_fmt(format_args!("CREATE TABLE {}foo(id INT)", schema_prefix)).unwrap();
  c.backend.execute(&_buffer).await.unwrap();
}

#[inline]
pub(crate) fn _generic_schema() -> AuxTestParams {
  AuxTestParams {
    default_schema: "",
    oapth_schema: "",
    schema_regulator: 2,
  }
}

#[inline]
pub(crate) async fn _migrate_doc_test<B>(_buffer: &mut String, c: &mut Commands<B>) -> MigrationGroupRef<'static>
where
  B: Backend,
{
  let mg = migration_group();
  c.migrate(_buffer, &mg, [migration()].iter()).await.unwrap();
  mg
}

#[inline]
pub(crate) fn _mssql_schema() -> AuxTestParams {
  AuxTestParams {
    default_schema: "dbo",
    oapth_schema: "_oapth",
    schema_regulator: 0,
  }
}

#[inline]
pub(crate) fn _pg_schema() -> AuxTestParams {
  AuxTestParams {
    default_schema: "public",
    oapth_schema: "_oapth",
    schema_regulator: 0,
  }
}
