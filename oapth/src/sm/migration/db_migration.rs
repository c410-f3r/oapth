mod db_migration_sqlx;

use crate::{
  sm::{MigrationCommon, MigrationGroup, Repeatability},
  DatabaseTy, Identifier,
};
use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
use core::fmt;

/// Migration retrieved from a database.
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DbMigration {
  common: MigrationCommon<Identifier>,
  created_on: DateTime<FixedOffset>,
  db_ty: DatabaseTy,
  group: MigrationGroup<Identifier>,
}

impl DbMigration {
  /// Data integrity
  #[inline]
  pub fn checksum(&self) -> u64 {
    self.common.checksum
  }

  /// When the migration was created.
  #[inline]
  pub fn created_on(&self) -> &DateTime<FixedOffset> {
    &self.created_on
  }

  /// See [DatabaseTy].
  #[inline]
  pub fn db_ty(&self) -> DatabaseTy {
    self.db_ty
  }

  /// Group
  #[inline]
  pub fn group(&self) -> &MigrationGroup<Identifier> {
    &self.group
  }

  /// Name
  #[inline]
  pub fn name(&self) -> &str {
    &self.common.name
  }

  /// Version
  #[inline]
  pub fn version(&self) -> i32 {
    self.common.version
  }
}

impl fmt::Display for DbMigration {
  #[inline]
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(fmt, "{}__{}", self.common.version, self.common.name)
  }
}

#[cfg(feature = "tiberius")]
impl crate::FromRow<tiberius::Row> for DbMigration {
  type Error = crate::Error;

  #[inline]
  fn from_row(from: &tiberius::Row) -> Result<Self, Self::Error> {
    macro_rules! translate {
      ($rslt:expr) => {
        $rslt?.ok_or_else(|| crate::Error::InvalidSqlQuery)?
      };
    }
    Ok(Self {
      common: MigrationCommon {
        checksum: {
          let checksum_str = translate!(from.try_get::<&str, _>("checksum"));
          _checksum_from_str(checksum_str)?
        },
        name: translate!(from.try_get::<&str, _>("name")).try_into()?,
        repeatability: _from_opt_i32(from.try_get("repeatability")?),
        version: translate!(from.try_get("version")),
      },
      created_on: {
        let s = translate!(from.try_get::<&str, _>("created_on"));
        _mssql_date_hack(s)?
      },
      db_ty: DatabaseTy::Mssql,
      group: MigrationGroup::new(
        translate!(from.try_get::<&str, _>("omg_name")).try_into()?,
        translate!(from.try_get("omg_version")),
      ),
    })
  }
}

fn _checksum_from_str(s: &str) -> crate::Result<u64> {
  s.parse().map_err(|_e| crate::Error::ChecksumMustBeANumber)
}

fn _fixed_from_naive_utc(naive: NaiveDateTime) -> DateTime<FixedOffset> {
  chrono::DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc).into()
}

fn _from_opt_i32(n: Option<i32>) -> Option<Repeatability> {
  match n {
    None => None,
    Some(0) => Some(Repeatability::Always),
    Some(_) => Some(Repeatability::OnChecksumChange),
  }
}

fn _mssql_date_hack(s: &str) -> crate::Result<DateTime<FixedOffset>> {
  let naive_rslt = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S");
  let naive = naive_rslt?;
  Ok(_fixed_from_naive_utc(naive))
}
