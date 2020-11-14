use crate::MigrationParams;
use alloc::string::String;
use core::iter::from_fn;
#[cfg(feature = "std")]
use {
  crate::{Migration, MigrationGroup},
  core::cmp::Ordering,
  std::{
    fs::{DirEntry, File},
    path::{Path, PathBuf},
  },
};

#[inline]
pub fn binary_seach_migration_by_version<T>(version: i32, migrations: &[T]) -> Option<(usize, &T)>
where
  T: MigrationParams,
{
  match migrations.binary_search_by(|m| {
    let common = m.common();
    common.version.cmp(&version)
  }) {
    Err(_) => None,
    #[
      // Indexing will not panic in this scenario
      allow(clippy::indexing_slicing)
    ]
    Ok(rslt) => Some((rslt, &migrations[rslt])),
  }
}

#[cfg(feature = "std")]
#[inline]
pub fn files(dir: &Path) -> crate::Result<impl Iterator<Item = crate::Result<DirEntry>>> {
  Ok(read_dir_with_cb(dir)?.filter_map(|entry_rslt| {
    let entry = entry_rslt.ok()?;
    let path = entry.path();
    if path.is_file() {
      Some(Ok(entry))
    } else {
      None
    }
  }))
}

#[cfg(feature = "std")]
#[inline]
pub fn group_and_migrations_from_path<F>(
  path: &Path,
  cb: F,
) -> Option<(MigrationGroup, impl Clone + Iterator<Item = crate::Result<Migration>>)>
where
  F: FnMut(&PathBuf, &PathBuf) -> Ordering,
{
  let (mg, mut migrations_vec) = migrations_from_dir(path)?;
  migrations_vec.sort_by(cb);
  let migrations = map_paths_into_migrations(migrations_vec.into_iter());
  Some((mg, migrations))
}

#[inline]
pub fn _group_dir_name_parts(s: &str) -> Option<(i32, String)> {
  if !s.is_ascii() {
    return None;
  }
  let mut split = s.split("__");
  let version = split.next()?.parse::<i32>().ok()?;
  let name = split.next()?.into();
  Some((version, name))
}

#[inline]
pub fn _iter_n_times<'a, I, T>(n: usize, iter: &'a mut I) -> impl Iterator<Item = T> + 'a
where
  I: Iterator<Item = T>,
{
  let mut counter: usize = 0;
  from_fn(move || {
    if counter >= n {
      return None;
    }
    counter = counter.saturating_add(1);
    iter.next()
  })
}

#[cfg(feature = "std")]
#[inline]
pub fn map_paths_into_migrations<I>(
  migrations: I,
) -> impl Clone + Iterator<Item = crate::Result<Migration>>
where
  I: Clone + Iterator<Item = PathBuf>,
{
  migrations.filter_map(|path| {
    let (version, name) = _migration_file_name_parts(path.file_name()?.to_str()?)?;
    let file = match File::open(path) {
      Err(e) => return Some(Err(e.into())),
      Ok(rslt) => rslt,
    };
    match crate::parse_migration(file) {
      Err(e) => Some(Err(e)),
      Ok((sql_up, sql_down)) => {
        let migration = Migration::new(version, name, sql_up, sql_down);
        Some(Ok(migration))
      }
    }
  })
}

#[inline]
pub fn _migration_file_name_parts(s: &str) -> Option<(i32, String)> {
  if !s.is_ascii() {
    return None;
  }
  let mut split = s.split("__");
  let version = split.next()?.parse::<i32>().ok()?;
  let name = split.next()?.strip_suffix(".sql")?.into();
  Some((version, name))
}

#[cfg(feature = "std")]
#[inline]
pub fn migrations_from_dir(path: &Path) -> Option<(MigrationGroup, Vec<PathBuf>)> {
  let (mg_version, mg_name) = _group_dir_name_parts(path.file_name()?.to_str()?)?;

  let files_rslts = files(path).ok()?.map(|rslt| rslt.map(|el| el.path()));

  let migration_paths = files_rslts.collect::<crate::Result<Vec<_>>>().ok()?;

  Some((crate::MigrationGroup::new(mg_version, mg_name), migration_paths))
}

#[cfg(feature = "std")]
#[inline]
fn read_dir_with_cb(dir: &Path) -> crate::Result<impl Iterator<Item = crate::Result<DirEntry>>> {
  Ok(std::fs::read_dir(dir)?.map(|entry_rslt| entry_rslt.map_err(|e| e.into())))
}

#[cfg(all(feature = "_integration_tests", test))]
macro_rules! create_integration_test {
  ($mod_name:ident, $backend:expr, $($fun:ident),+) => {
    mod $mod_name {
      $(
        #[tokio::test]
        async fn $fun() {
          let _ = env_logger::builder().is_test(true).try_init();
          let mut commands = crate::Commands::new($backend);
          super::$fun(&mut commands).await;
        }
      )+
    }
  };
}

#[cfg(all(feature = "_integration_tests", test))]
macro_rules! _create_integration_test_backend {
  ($backend_ty:ident) => {{
    let c = crate::Config::with_url_from_default_var().unwrap();
    crate::$backend_ty::new(&c).await.unwrap()
  }};
}

#[cfg(all(feature = "_integration_tests", test))]
macro_rules! create_integration_tests {
  ($($fun:ident),+) => {
    #[cfg(feature = "with-diesel-mysql")]
    create_integration_test!(
      diesel_mysql,
      _create_integration_test_backend!(DieselMysql),
      $($fun),+
    );

    #[cfg(feature = "with-diesel-postgres")]
    create_integration_test!(
      diesel_postgres,
      _create_integration_test_backend!(DieselPostgres),
      $($fun),+
    );

    #[cfg(feature = "with-diesel-sqlite")]
    create_integration_test!(
      diesel_sqlite,
      _create_integration_test_backend!(DieselSqlite),
      $($fun),+
    );

    #[cfg(feature = "with-mysql_async")]
    create_integration_test!(
      mysql_async,
      _create_integration_test_backend!(MysqlAsync),
      $($fun),+
    );

    #[cfg(feature = "with-rusqlite")]
    create_integration_test!(
      rusqlite,
      _create_integration_test_backend!(Rusqlite),
      $($fun),+
    );

    #[cfg(feature = "with-sqlx-mssql")]
    create_integration_test!(
      sqlx_mssql,
      _create_integration_test_backend!(SqlxMssql),
      $($fun),+
    );

    #[cfg(feature = "with-sqlx-mysql")]
    create_integration_test!(
      sqlx_mysql,
      _create_integration_test_backend!(SqlxMysql),
      $($fun),+
    );

    #[cfg(feature = "with-sqlx-postgres")]
    create_integration_test!(
      sqlx_postgres,
      _create_integration_test_backend!(SqlxPostgres),
      $($fun),+
    );

    #[cfg(feature = "with-sqlx-sqlite")]
    create_integration_test!(
      sqlx_sqlite,
      _create_integration_test_backend!(SqlxSqlite),
      $($fun),+
    );

    #[cfg(feature = "with-tiberius")]
    create_integration_test!(
      tiberius,
      {
        use tokio_util::compat::Tokio02AsyncWriteCompatExt;
        let c = crate::Config::with_url_from_default_var().unwrap();
        let tcp = tokio::net::TcpStream::connect(c.full_host().unwrap()).await.unwrap();
        crate::Tiberius::new(&c, tcp.compat_write()).await.unwrap()
      },
      $($fun),+
    );

    #[cfg(feature = "with-tokio-postgres")]
    create_integration_test!(
      tokio_postgres,
      _create_integration_test_backend!(TokioPostgres),
      $($fun),+
    );
  };
}

#[cfg(feature = "std")]
macro_rules! loop_files {
  ($buffer:expr, $iter:expr, $n:expr, $cb:expr) => {{
    loop {
      for el in crate::_iter_n_times($n, &mut $iter) {
        $buffer.push(el?);
      }
      if $buffer.is_empty() {
        break;
      }
      $cb;
      $buffer.clear();
    }
  }};
}
