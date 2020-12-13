use crate::{DbMigration, Migration};

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

#[inline]
pub(crate) fn migration_is_divergent(db_migrations: &[DbMigration], migration: &Migration) -> bool {
  let version = migration.version();
  let opt = binary_seach_migration_by_version(version, &db_migrations);
  let db_migration = if let Some(rslt) = opt {
    rslt
  } else {
    return false;
  };
  migration.checksum() != db_migration.checksum()
    || migration.name() != db_migration.name()
    || migration.version() != db_migration.version()
}

#[oapth_macros::std_]
mod std {
  use crate::{parse_migration_cfg, parse_unified_migration, Migration, MigrationGroup};
  use alloc::string::String;
  use arrayvec::ArrayString;
  use core::{cmp::Ordering, fmt::Write, iter::from_fn};
  use std::{
    fs::{read_to_string, DirEntry, File},
    path::{Path, PathBuf},
  };

  #[inline]
  pub fn files(dir: &Path) -> crate::Result<impl Iterator<Item = crate::Result<DirEntry>>> {
    Ok(read_dir(dir)?.filter_map(|entry_rslt| {
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
      if path.is_dir() {
        let dir_name = path.file_name()?.to_str()?;
        let (version, name) = migration_file_name_parts(dir_name)?;

        let mut cfg_file_name = ArrayString::<[u8; 64]>::new();
        cfg_file_name.write_fmt(format_args!("{}.cfg", dir_name)).ok()?;

        let mut down_file_name = ArrayString::<[u8; 64]>::new();
        down_file_name.write_fmt(format_args!("{}_down.sql", dir_name)).ok()?;

        let mut up_file_name = ArrayString::<[u8; 64]>::new();
        up_file_name.write_fmt(format_args!("{}_up.sql", dir_name)).ok()?;

        let mut cfg = Default::default();
        let mut sql_down = String::new();
        let mut sql_up = String::new();

        for file_rslt in files(path.as_path()).ok()? {
          let file = file_rslt.ok()?;
          let file_path = file.path();
          let file_name = file_path.file_name()?.to_str()?;
          if file_name == &cfg_file_name {
            let file = match File::open(file_path) {
              Err(e) => return Some(Err(e.into())),
              Ok(rslt) => rslt,
            };
            cfg = parse_migration_cfg(file).ok()?;
          } else if file_name == &down_file_name {
            sql_down = read_to_string(file_path).ok()?;
          } else if file_name == &up_file_name {
            sql_up = read_to_string(file_path).ok()?;
          } else {
            continue;
          }
        }

        Some(Ok(Migration::new(
          cfg.dbs.into_iter(),
          cfg.repeatability,
          version,
          name,
          sql_up,
          sql_down,
        )))
      } else if let Some(Some(file_name)) = path.file_name().map(|e| e.to_str()) {
        let (version, name) = migration_file_name_parts(file_name)?;
        let file = match File::open(path) {
          Err(e) => return Some(Err(e.into())),
          Ok(rslt) => rslt,
        };
        let parsed_migration = match parse_unified_migration(file) {
          Err(e) => return Some(Err(e)),
          Ok(rslt) => rslt,
        };
        Some(Ok(Migration::new(
          parsed_migration.cfg.dbs.into_iter(),
          parsed_migration.cfg.repeatability,
          version,
          name,
          parsed_migration.sql_up,
          parsed_migration.sql_down,
        )))
      } else {
        None
      }
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
    let migration_paths = read_dir(path)
      .ok()?
      .filter_map(|entry_rslt| Some(entry_rslt.ok()?.path()))
      .collect::<Vec<PathBuf>>();
    Some((crate::MigrationGroup::new(mg_version, mg_name), migration_paths))
  }

  #[inline]
  fn read_dir(dir: &Path) -> crate::Result<impl Iterator<Item = crate::Result<DirEntry>>> {
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
