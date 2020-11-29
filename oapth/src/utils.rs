use crate::DbMigration;

#[oapth_macros::std_]
macro_rules! loop_files {
  ($buffer:expr, $iter:expr, $n:expr, $cb:expr) => {{
    loop {
      for el in crate::iter_n_times($n, &mut $iter) {
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

#[oapth_macros::std_]
mod std {
  use crate::{Migration, MigrationGroup};
  use alloc::string::String;
  use core::{cmp::Ordering, iter::from_fn};
  use std::{
    fs::{DirEntry, File},
    path::{Path, PathBuf},
  };

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
  pub fn group_and_migrations_from_path<F>(
    path: &Path,
    cb: F,
  ) -> Option<(MigrationGroup, impl Clone + Iterator<Item = crate::Result<Migration>>)>
  where
    F: FnMut(&PathBuf, &PathBuf) -> Ordering,
  {
    let (mg, mut migrations_vec) = migrations_from_dir(path)?;
    migrations_vec.sort_by(cb);
    let migrations = migrations_vec.into_iter().filter_map(move |path| {
      let (version, name) = migration_file_name_parts(path.file_name()?.to_str()?)?;
      let file = match File::open(path) {
        Err(e) => return Some(Err(e.into())),
        Ok(rslt) => rslt,
      };
      let parsed_migration = match crate::parse_migration(file) {
        Err(e) => return Some(Err(e)),
        Ok(rslt) => rslt,
      };
      Some(Ok(Migration::new(
        parsed_migration.dbs.into_iter(),
        version,
        name,
        parsed_migration.sql_up,
        parsed_migration.sql_down,
      )))
    });
    Some((mg, migrations))
  }

  #[inline]
  pub fn group_dir_name_parts(s: &str) -> Option<(i32, String)> {
    if !s.is_ascii() {
      return None;
    }
    let mut split = s.split("__");
    let version = split.next()?.parse::<i32>().ok()?;
    let name = split.next()?.into();
    Some((version, name))
  }

  #[inline]
  pub fn iter_n_times<'a, I, T>(n: usize, iter: &'a mut I) -> impl Iterator<Item = T> + 'a
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

  #[inline]
  fn migrations_from_dir(path: &Path) -> Option<(MigrationGroup, Vec<PathBuf>)> {
    let (mg_version, mg_name) = group_dir_name_parts(path.file_name()?.to_str()?)?;

    let files_rslts = files(path).ok()?.map(|rslt| rslt.map(|el| el.path()));

    let migration_paths = files_rslts.collect::<crate::Result<Vec<_>>>().ok()?;

    Some((crate::MigrationGroup::new(mg_version, mg_name), migration_paths))
  }

  #[inline]
  fn read_dir_with_cb(dir: &Path) -> crate::Result<impl Iterator<Item = crate::Result<DirEntry>>> {
    Ok(std::fs::read_dir(dir)?.map(|entry_rslt| entry_rslt.map_err(|e| e.into())))
  }
}

#[oapth_macros::std_]
pub use self::std::*;

#[inline]
pub fn binary_seach_migration_by_version(
  version: i32,
  migrations: &[DbMigration],
) -> Option<&DbMigration> {
  match migrations.binary_search_by(|m| m.version().cmp(&version)) {
    Err(_) => None,
    Ok(rslt) => migrations.get(rslt),
  }
}
