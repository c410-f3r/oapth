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
pub fn binary_seach_migration_by_version<T>(version: i32, migrations: &[T]) -> Option<(usize, &T)>
where
  T: MigrationParams,
{
  match migrations.binary_search_by(|m| {
    let common = m.common();
    common.version.cmp(&version)
  }) {
    Err(_) => None,
    #[allow(
      // Indexing will not panic in this scenario
      clippy::indexing_slicing
    )]
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
fn map_paths_into_migrations<I>(
  migrations: I,
) -> impl Clone + Iterator<Item = crate::Result<Migration>>
where
  I: Clone + Iterator<Item = PathBuf>,
{
  migrations.filter_map(|path| {
    let (version, name) = migration_file_name_parts(path.file_name()?.to_str()?)?;
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

#[cfg(feature = "std")]
#[inline]
fn migration_file_name_parts(s: &str) -> Option<(i32, String)> {
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
fn migrations_from_dir(path: &Path) -> Option<(MigrationGroup, Vec<PathBuf>)> {
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
