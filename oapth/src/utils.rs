use crate::MigrationParams;
use alloc::string::String;
use core::iter::from_fn;
#[cfg(feature = "std")]
use {
  crate::Migration,
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

#[cfg(all(feature = "_integration_tests", test))]
macro_rules! create_integration_test {
  ($backend:ident, $($fun:ident),+) => {
    #[allow(non_snake_case)]
    mod $backend {
      $(
        #[tokio::test]
        async fn $fun() {
          let _ = env_logger::builder().is_test(true).try_init();
          let c = crate::Config::with_url_from_default_var().unwrap();
          let backend = crate::$backend::new(&c).await.unwrap();
          let commands = crate::Commands::new(backend);
          super::$fun(commands).await;
        }
      )+
    }
  };
}

#[cfg(all(feature = "_integration_tests", test))]
macro_rules! create_integration_tests {
  ($($fun:ident),+) => {
    #[cfg(feature = "with-mysql_async")]
    create_integration_test!(MysqlAsync, $($fun),+);
    #[cfg(feature = "with-rusqlite")]
    create_integration_test!(Rusqlite, $($fun),+);
    #[cfg(feature = "with-sqlx-mssql")]
    create_integration_test!(SqlxMssql, $($fun),+);
    #[cfg(feature = "with-sqlx-mysql")]
    create_integration_test!(SqlxMysql, $($fun),+);
    #[cfg(feature = "with-sqlx-postgres")]
    create_integration_test!(SqlxPostgres, $($fun),+);
    #[cfg(feature = "with-sqlx-sqlite")]
    create_integration_test!(SqlxSqlite, $($fun),+);
    #[cfg(feature = "with-tokio-postgres")]
    create_integration_test!(TokioPostgres, $($fun),+);
  };
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
pub fn scan_canonical_migrations_dir(
  dir: &Path,
) -> crate::Result<Vec<(crate::MigrationGroup, Vec<PathBuf>)>> {
  let iter = read_dir_with_cb(dir)?.filter_map(|entry_rslt| {
    let entry = entry_rslt.ok()?;

    let path = entry.path();
    if !path.is_dir() {
      return None;
    }

    let (group_version, group_name) = _group_dir_name_parts(path.as_path().file_name()?.to_str()?)?;

    let files_rslts = match files(path.as_path()) {
      Err(e) => return Some(Err(e)),
      Ok(iter) => iter.map(|entry_rslt| {
        let entry = match entry_rslt {
          Err(e) => return Err(e),
          Ok(rslt) => rslt,
        };
        Ok(entry.path())
      }),
    };

    let migration_paths = match files_rslts.collect::<crate::Result<Vec<_>>>() {
      Err(e) => return Some(Err(e)),
      Ok(rslt) => rslt,
    };

    Some(Ok((crate::MigrationGroup::new(group_version, group_name), migration_paths)))
  });
  iter.collect::<crate::Result<Vec<_>>>()
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

#[inline]
fn _group_dir_name_parts(s: &str) -> Option<(i32, String)> {
  if !s.is_ascii() {
    return None;
  }
  let mut split = s.split("__");
  let version = split.next()?.parse::<i32>().ok()?;
  let name = split.next()?.into();
  Some((version, name))
}

#[cfg(feature = "std")]
#[inline]
fn read_dir_with_cb(dir: &Path) -> crate::Result<impl Iterator<Item = crate::Result<DirEntry>>> {
  Ok(std::fs::read_dir(dir)?.map(|entry_rslt| entry_rslt.map_err(|e| e.into())))
}
