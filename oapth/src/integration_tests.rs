mod back_end;
mod db;
mod generic;
mod schema;

use crate::{
  doc_tests::{migration, migration_group},
  BackEnd, Commands, MigrationGroup,
};
use arrayvec::ArrayString;
use core::fmt::Write;

macro_rules! create_integration_test {
  ($back_end:expr, $aux:expr, $($fun:path),*) => {{
    $(
      let mut commands = crate::Commands::new($back_end);
      commands.clean().await.unwrap();
      $fun(&mut commands, $aux).await;
    )*
  }};
}

macro_rules! _create_integration_test_back_end {
  ($back_end_ty:ident) => {{
    let c = crate::Config::with_url_from_default_var().unwrap();
    crate::$back_end_ty::new(&c).await.unwrap()
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
    pub async fn $fn_name() {
      oapth_macros::diesel_mysql! {
        create_integration_test!(
          _create_integration_test_back_end!(DieselMysql),
          _generic_schema(),
          $($diesel_mysql),*
        );
      }
      oapth_macros::diesel_pg! {
        create_integration_test!(
          _create_integration_test_back_end!(DieselPg),
          _pg_schema(),
          $($diesel_pg),*
        );
      }

      oapth_macros::diesel_sqlite! {
        create_integration_test!(
          _create_integration_test_back_end!(DieselSqlite),
          _generic_schema(),
          $($diesel_sqlite),*
        );
      }

      oapth_macros::mysql_async! {
        create_integration_test!(
          _create_integration_test_back_end!(MysqlAsync),
          _generic_schema(),
          $($mysql_async),*
        );
      }

      oapth_macros::rusqlite! {
        create_integration_test!(
          _create_integration_test_back_end!(Rusqlite),
          _generic_schema(),
          $($rusqlite),*
        );
      }

      oapth_macros::sqlx_mssql! {
        create_integration_test!(
          _create_integration_test_back_end!(SqlxMssql),
          _mssql_schema(),
          $($sqlx_mssql),*
        );
      }

      oapth_macros::sqlx_mysql! {
        create_integration_test!(
          _create_integration_test_back_end!(SqlxMysql),
          _generic_schema(),
          $($sqlx_mysql),*
        );
      }

      oapth_macros::sqlx_pg! {
        create_integration_test!(
          _create_integration_test_back_end!(SqlxPg),
          _pg_schema(),
          $($sqlx_pg),*
        );
      }

      oapth_macros::sqlx_sqlite! {
        create_integration_test!(
          _create_integration_test_back_end!(SqlxSqlite),
          _generic_schema(),
          $($sqlx_sqlite),*
        );
      }

      oapth_macros::tiberius! {
        create_integration_test!(
          {
            use tokio_util::compat::Tokio02AsyncWriteCompatExt;
            let c = crate::Config::with_url_from_default_var().unwrap();
            let tcp = tokio::net::TcpStream::connect(c.full_host().unwrap()).await.unwrap();
            crate::Tiberius::new(&c, tcp.compat_write()).await.unwrap()
          },
          _mssql_schema(),
          $($tiberius),*
        );
      }

      oapth_macros::tokio_postgres! {
        create_integration_test!(
          _create_integration_test_back_end!(TokioPostgres),
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
      integration_tests_back_end,
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

      integration_tests_back_end().await;
      integration_tests_db().await;
      integration_tests_generic().await;
      integration_tests_schema().await;
    }
  };
}

create_all_integration_tests!(
  // Back end

  diesel_mysql:
    back_end::_back_end_has_migration_with_utc_time;
  diesel_pg:
    back_end::_back_end_has_migration_with_utc_time;
  diesel_sqlite:
    back_end::_back_end_has_migration_with_utc_time;
  mysql_async:
    back_end::_back_end_has_migration_with_utc_time;
  rusqlite:
    back_end::_back_end_has_migration_with_utc_time;
  sqlx_mssql:
    back_end::_back_end_has_migration_with_utc_time;
  sqlx_mysql:
    back_end::_back_end_has_migration_with_utc_time;
  sqlx_pg: ;
  sqlx_sqlite:
    back_end::_back_end_has_migration_with_utc_time;
  tiberius:
    back_end::_back_end_has_migration_with_utc_time;
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
pub struct AuxTestParams {
  pub default_schema: &'static str,
  pub default_schema_prefix: &'static str,
  pub oapth_schema: &'static str,
  pub oapth_schema_prefix: &'static str,
  pub schema_regulator: usize,
}

pub async fn create_foo_table<B>(c: &mut Commands<B>, schema_prefix: &str)
where
  B: BackEnd,
{
  let mut buffer = ArrayString::<[u8; 64]>::new();
  buffer.write_fmt(format_args!("CREATE TABLE {}foo(id INT)", schema_prefix)).unwrap();
  c.back_end.execute(&buffer).await.unwrap();
}

#[inline]
pub fn _generic_schema() -> AuxTestParams {
  AuxTestParams {
    default_schema: "",
    default_schema_prefix: "",
    oapth_schema: "",
    oapth_schema_prefix: "",
    schema_regulator: 2,
  }
}

#[inline]
pub async fn _migrate_doc_test<B>(c: &mut Commands<B>) -> MigrationGroup
where
  B: BackEnd
{
  let mg = migration_group();
  c.migrate(&mg, [migration()].iter()).await.unwrap();
  mg
}

#[inline]
pub fn _mssql_schema() -> AuxTestParams {
  AuxTestParams {
    default_schema: "dbo",
    default_schema_prefix: "dbo.",
    oapth_schema: "_oapth",
    oapth_schema_prefix: "_oapth.",
    schema_regulator: 0,
  }
}

#[inline]
pub fn _pg_schema() -> AuxTestParams {
  AuxTestParams {
    default_schema: "public",
    default_schema_prefix: "public.",
    oapth_schema: "_oapth",
    oapth_schema_prefix: "public.",
    schema_regulator: 0,
  }
}
