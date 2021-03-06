use crate::{parse_migration_cfg, parse_unified_migration, Database, Repeatability};
use arrayvec::ArrayString;
use core::{
  cmp::Ordering,
  fmt::Write,
  hash::{Hash, Hasher},
};
use siphasher::sip::SipHasher13;
use std::{
  fs::{read_to_string, DirEntry, File},
  path::{Path, PathBuf},
};

type MigrationGroupParts = (String, i32);
type MigrationParts = (u64, Vec<Database>, String, Option<Repeatability>, String, String, i32);

macro_rules! opt_to_inv_mig {
  ($opt:expr) => {
    $opt().ok_or_else(|| crate::Error::InvalidMigration)
  };
}

/// Calculate checksum
#[inline]
pub fn calc_checksum(name: &str, sql_up: &str, sql_down: &str, version: i32) -> u64 {
  let mut hasher = SipHasher13::new();
  name.hash(&mut hasher);
  sql_up.hash(&mut hasher);
  sql_down.hash(&mut hasher);
  version.hash(&mut hasher);
  hasher.finish()
}

/// All files of a given `path`.
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

/// A group and all of its migrations of a given `path`.
#[inline]
pub fn group_and_migrations_from_path<F>(
  path: &Path,
  cb: F,
) -> crate::Result<(MigrationGroupParts, impl Clone + Iterator<Item = crate::Result<MigrationParts>>)>
where
  F: FnMut(&PathBuf, &PathBuf) -> Ordering,
{
  let (mg, mut migrations_vec) = migrations_from_dir(path)?;
  migrations_vec.sort_by(cb);
  let migrations = migrations_vec.into_iter().map(move |path| {
    let checksum;
    let mut dbs = Default::default();
    let name;
    let mut repeatability = Default::default();
    let mut sql_down = Default::default();
    let mut sql_up = Default::default();
    let version;

    if path.is_dir() {
      let dir_name = opt_to_inv_mig!(|| path.file_name()?.to_str())?;
      let parts = dir_name_parts(dir_name)?;
      name = parts.0;
      version = parts.1;

      let mut cfg_file_name = ArrayString::<[u8; 64]>::new();
      cfg_file_name.write_fmt(format_args!("{}.cfg", dir_name))?;

      let mut down_file_name = ArrayString::<[u8; 64]>::new();
      down_file_name.write_fmt(format_args!("{}_down.sql", dir_name))?;

      let mut up_file_name = ArrayString::<[u8; 64]>::new();
      up_file_name.write_fmt(format_args!("{}_up.sql", dir_name))?;

      for file_rslt in files(path.as_path())? {
        let file = file_rslt?;
        let file_path = file.path();
        let file_name = opt_to_inv_mig!(|| file_path.file_name()?.to_str())?;
        if file_name == &cfg_file_name {
          let mc = parse_migration_cfg(File::open(file_path)?)?;
          dbs = mc.dbs;
          repeatability = mc.repeatability;
        } else if file_name == &down_file_name {
          sql_down = read_to_string(file_path)?;
        } else if file_name == &up_file_name {
          sql_up = read_to_string(file_path)?;
        } else {
          continue;
        }
      }
    } else if let Some(Some(file_name)) = path.file_name().map(|e| e.to_str()) {
      let parts = migration_file_name_parts(file_name)?;
      name = parts.0;
      version = parts.1;
      let pm = parse_unified_migration(File::open(path)?)?;
      dbs = pm.cfg.dbs;
      repeatability = pm.cfg.repeatability;
      sql_up = pm.sql_up;
      sql_down = pm.sql_down;
    } else {
      return Err(crate::Error::InvalidMigration);
    }

    checksum = calc_checksum(&name, &sql_up, &sql_down, version);

    Ok((checksum, dbs, name, repeatability, sql_down, sql_up, version))
  });

  Ok((mg, migrations))
}

#[inline]
pub(crate) fn dir_name_parts(s: &str) -> crate::Result<(String, i32)> {
  let f = || {
    if !s.is_ascii() {
      return None;
    }
    let mut split = s.split("__");
    let version = split.next()?.parse::<i32>().ok()?;
    let name = split.next()?.into();
    Some((name, version))
  };
  f().ok_or(crate::Error::InvalidMigration)
}

#[inline]
fn migration_file_name_parts(s: &str) -> crate::Result<(String, i32)> {
  let f = || {
    if !s.is_ascii() {
      return None;
    }
    let mut split = s.split("__");
    let version = split.next()?.parse::<i32>().ok()?;
    let name = split.next()?.strip_suffix(".sql")?.into();
    Some((name, version))
  };
  f().ok_or(crate::Error::InvalidMigration)
}

#[inline]
fn migrations_from_dir(path: &Path) -> crate::Result<(MigrationGroupParts, Vec<PathBuf>)> {
  let path_str = opt_to_inv_mig!(|| path.file_name()?.to_str())?;
  let (mg_name, mg_version) = dir_name_parts(path_str)?;
  let migration_paths = read_dir(path)?
    .map(|entry_rslt| Ok(entry_rslt?.path()))
    .collect::<crate::Result<Vec<PathBuf>>>()?;
  Ok(((mg_name, mg_version), migration_paths))
}

#[inline]
fn read_dir(dir: &Path) -> crate::Result<impl Iterator<Item = crate::Result<DirEntry>>> {
  Ok(std::fs::read_dir(dir)?.map(|entry_rslt| entry_rslt.map_err(|e| e.into())))
}
