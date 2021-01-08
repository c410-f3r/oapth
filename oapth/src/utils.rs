use crate::{DbMigration, MigrationRef};
#[oapth_macros::_std]
use {
  crate::{MigrationCommon, MigrationGroupOwned, MigrationOwned},
  core::cmp::Ordering,
  std::path::{Path, PathBuf},
};

#[oapth_macros::_std]
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
pub(crate) fn binary_seach_migration_by_version(
  version: i32,
  migrations: &[DbMigration],
) -> Option<&DbMigration> {
  match migrations.binary_search_by(|m| m.version().cmp(&version)) {
    Err(_) => None,
    Ok(rslt) => migrations.get(rslt),
  }
}

#[inline]
pub(crate) fn is_migration_divergent(
  db_migrations: &[DbMigration],
  migration: &MigrationRef<'_, '_>,
) -> bool {
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

#[oapth_macros::_std]
#[inline]
pub(crate) fn iter_n_times<'a, 'b, I, T>(n: usize, iter: &'a mut I) -> impl Iterator<Item = T> + 'b
where
  'a: 'b,
  I: Iterator<Item = T>,
{
  let mut counter: usize = 0;
  core::iter::from_fn(move || {
    if counter >= n {
      return None;
    }
    counter = counter.saturating_add(1);
    iter.next()
  })
}

#[oapth_macros::_std]
#[inline]
pub(crate) fn group_and_migrations_from_path<F>(
  path: &Path,
  cb: F,
) -> crate::Result<(MigrationGroupOwned, impl Clone + Iterator<Item = crate::Result<MigrationOwned>>)>
where
  F: FnMut(&PathBuf, &PathBuf) -> Ordering,
{
  let ((mg_name, mg_version), ms) = oapth_commons::group_and_migrations_from_path(path, cb)?;
  let mg = MigrationGroupOwned { name: mg_name, version: mg_version };
  let mapped = ms.map(|rslt| {
    let (checksum, dbs, name, repeatability, sql_down, sql_up, version) = rslt?;
    Ok(MigrationOwned {
      dbs,
      common: MigrationCommon { checksum, name, repeatability, version },
      sql_down,
      sql_up,
    })
  });
  Ok((mg, mapped))
}
