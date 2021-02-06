oapth_macros::_diesel_! { mod db_migration_diesel; }
oapth_macros::_sqlx_! { mod db_migration_sqlx; }

use crate::{MigrationCommonOwned, MigrationGroupOwned};
use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
use core::fmt;
use oapth_commons::Database;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DbMigration {
  common: MigrationCommonOwned,
  created_on: DateTime<FixedOffset>,
  db: Database,
  group: MigrationGroupOwned,
}

impl DbMigration {
  #[inline]
  pub fn checksum(&self) -> u64 {
    self.common.checksum
  }

  #[inline]
  pub fn created_on(&self) -> &DateTime<FixedOffset> {
    &self.created_on
  }

  #[inline]
  pub fn db(&self) -> Database {
    self.db
  }

  #[inline]
  pub fn group(&self) -> &MigrationGroupOwned {
    &self.group
  }

  #[inline]
  pub fn name(&self) -> &str {
    &self.common.name
  }

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

#[oapth_macros::_mysql_async]
impl core::convert::TryFrom<mysql_async::Row> for DbMigration {
  type Error = crate::Error;

  #[inline]
  fn try_from(from: mysql_async::Row) -> Result<Self, Self::Error> {
    macro_rules! translate {
      ($ty:ty, $name:expr) => {
        from.get::<$ty, _>($name).ok_or_else(|| {
          crate::Error::MysqlAsync(
            mysql_async::Error::Driver(mysql_async::DriverError::MissingNamedParam {
              name: $name.into(),
            })
            .into(),
          )
        })
      };
    }
    Ok(Self {
      common: MigrationCommonOwned {
        checksum: checksum_from_str(&translate!(String, "checksum")?)?,
        name: translate!(_, "name")?,
        repeatability: from_opt_i32(translate!(_, "repeatability")?),
        version: translate!(_, "version")?,
      },
      created_on: {
        let ndt = translate!(NaiveDateTime, "created_on")?;
        _fixed_from_naive_utc(ndt)
      },
      db: Database::Mysql,
      group: MigrationGroupOwned {
        version: translate!(_, "omg_version")?,
        name: translate!(_, "omg_name")?,
      },
    })
  }
}

#[oapth_macros::_rusqlite]
impl<'a, 'b> core::convert::TryFrom<&'a rusqlite::Row<'b>> for DbMigration {
  type Error = crate::Error;

  #[inline]
  fn try_from(from: &'a rusqlite::Row<'b>) -> Result<Self, Self::Error> {
    Ok(Self {
      common: MigrationCommonOwned {
        checksum: checksum_from_str(&from.get::<_, String>("checksum")?)?,
        name: from.get("name")?,
        repeatability: from_opt_i32(from.get("repeatability")?),
        version: from.get("version")?,
      },
      created_on: from.get::<_, DateTime<Utc>>("created_on")?.into(),
      db: Database::Sqlite,
      group: MigrationGroupOwned { version: from.get("omg_version")?, name: from.get("omg_name")? },
    })
  }
}

#[oapth_macros::_tiberius]
impl core::convert::TryFrom<tiberius::Row> for DbMigration {
  type Error = crate::Error;

  #[inline]
  fn try_from(from: tiberius::Row) -> Result<Self, Self::Error> {
    macro_rules! translate {
      ($rslt:expr) => {
        $rslt?.ok_or_else(|| crate::Error::Other("Invalid row for migration retrieval"))?
      };
    }
    Ok(Self {
      common: MigrationCommonOwned {
        checksum: {
          let checksum_str = translate!(from.try_get::<&str, _>("checksum"));
          checksum_from_str(checksum_str)?
        },
        name: translate!(from.try_get::<&str, _>("name")).into(),
        repeatability: from_opt_i32(from.try_get("repeatability")?),
        version: translate!(from.try_get("version")),
      },
      created_on: {
        let s = translate!(from.try_get::<&str, _>("created_on"));
        mssql_date_hack(s)?
      },
      db: Database::Mssql,
      group: MigrationGroupOwned {
        version: translate!(from.try_get("omg_version")),
        name: translate!(from.try_get::<&str, _>("omg_name")).into(),
      },
    })
  }
}

#[oapth_macros::_tokio_postgres]
impl core::convert::TryFrom<tokio_postgres::Row> for DbMigration {
  type Error = crate::Error;

  #[inline]
  fn try_from(from: tokio_postgres::Row) -> Result<Self, Self::Error> {
    Ok(Self {
      common: MigrationCommonOwned {
        checksum: checksum_from_str(from.try_get("checksum")?)?,
        name: from.try_get("name")?,
        repeatability: from_opt_i32(from.try_get("repeatability")?),
        version: from.try_get("version")?,
      },
      created_on: from.try_get("created_on")?,
      db: Database::Pg,
      group: MigrationGroupOwned {
        version: from.try_get("omg_version")?,
        name: from.try_get("omg_name")?,
      },
    })
  }
}

#[oapth_macros::_any_db]
fn checksum_from_str(s: &str) -> crate::Result<u64> {
  s.parse().map_err(|_e| crate::Error::Other("Database checksum is not a number"))
}

fn _fixed_from_naive_utc(naive: NaiveDateTime) -> DateTime<FixedOffset> {
  chrono::DateTime::<Utc>::from_utc(naive, Utc).into()
}

#[oapth_macros::_any_db]
const fn from_opt_i32(n: Option<i32>) -> Option<oapth_commons::Repeatability> {
  match n {
    None => None,
    Some(0) => Some(oapth_commons::Repeatability::Always),
    Some(_) => Some(oapth_commons::Repeatability::OnChecksumChange),
  }
}

#[oapth_macros::_mssql]
fn mssql_date_hack(s: &str) -> crate::Result<DateTime<FixedOffset>> {
  let naive_rslt = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S");
  let naive = naive_rslt.map_err(|_| crate::Error::Other("Invalid date for mssql"))?;
  Ok(_fixed_from_naive_utc(naive))
}
