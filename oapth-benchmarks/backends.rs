#![allow(missing_docs)]

use criterion::{
  criterion_group, criterion_main, measurement::Measurement, BenchmarkGroup, BenchmarkId, Criterion,
};
use oapth::{
  database::{SqlxMysql, SqlxPostgres, SqlxSqlite, Tiberius},
  sm::Commands,
  Config,
};
use std::path::Path;
use tokio::runtime::Runtime;

macro_rules! add_benchmark_group {
  (
    $criterion:expr,
    $f:ident,
    $sqlx_mysql:expr,
    $sqlx_pg:expr,
    $sqlx_sqlite:expr,
    $tiberius:expr $(,)?
  ) => {
    fn $f<M>(group: &mut BenchmarkGroup<'_, M>, size: usize)
    where
      M: Measurement,
    {
      let mssql_config = Config::with_url_from_var("MSSQL").unwrap();
      let mysql_config = Config::with_url_from_var("MYSQL").unwrap();
      let pg_config = Config::with_url_from_var("POSTGRES").unwrap();
      let sqlite_config = Config::with_url_from_var("SQLITE").unwrap();

      group.bench_with_input(BenchmarkId::new("SQLx - MySql", size), &size, |b, _| {
        b.iter(|| {
          let rt = Runtime::new().unwrap();
          rt.block_on(async {
            let c = Commands::with_database(SqlxMysql::new(&mysql_config).await.unwrap());
            $sqlx_mysql(c).await;
          });
        })
      });

      group.bench_with_input(BenchmarkId::new("SQLx - PostgreSQL", size), &size, |b, _| {
        b.iter(|| {
          let rt = Runtime::new().unwrap();
          rt.block_on(async {
            let c = Commands::with_database(SqlxPostgres::new(&pg_config).await.unwrap());
            $sqlx_pg(c).await;
          });
        })
      });

      group.bench_with_input(BenchmarkId::new("SQLx - SQLite", size), &size, |b, _| {
        b.iter(|| {
          let rt = Runtime::new().unwrap();
          rt.block_on(async {
            let c = Commands::with_database(SqlxSqlite::new(&sqlite_config).await.unwrap());
            $sqlx_sqlite(c).await;
          });
        })
      });

      group.bench_with_input(BenchmarkId::new("tiberius", size), &size, |b, _| {
        b.iter(|| {
          let rt = Runtime::new().unwrap();
          rt.block_on(async {
            use tokio_util::compat::TokioAsyncWriteCompatExt;
            let tcp =
              tokio::net::TcpStream::connect(mssql_config.full_host().unwrap()).await.unwrap();
            let c = Commands::with_database(
              Tiberius::new(&mssql_config, tcp.compat_write()).await.unwrap(),
            );
            $tiberius(c).await;
          });
        })
      });
    }

    let mut group = $criterion.benchmark_group(stringify!($f));
    $f(&mut group, 32);
    group.finish();
  };
}

fn criterion_benchmark(c: &mut Criterion) {
  macro_rules! path {
    () => {
      Path::new("../.test-utils/migrations.toml")
    };
  }
  add_benchmark_group!(
    c,
    migrate,
    |mut c: Commands<SqlxMysql>| async move {
      c.migrate_from_toml_path((&mut String::new(), &mut Vec::new()), path!()).await.unwrap();
    },
    |mut c: Commands<SqlxPostgres>| async move {
      c.migrate_from_toml_path((&mut String::new(), &mut Vec::new()), path!()).await.unwrap();
    },
    |mut c: Commands<SqlxSqlite>| async move {
      c.migrate_from_toml_path((&mut String::new(), &mut Vec::new()), path!()).await.unwrap();
    },
    |mut c: Commands<Tiberius<_>>| async move {
      c.migrate_from_toml_path((&mut String::new(), &mut Vec::new()), path!()).await.unwrap();
    }
  );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
