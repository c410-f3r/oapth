#[cfg(any(
  feature = "with-diesel-mysql",
  feature = "with-diesel-postgres",
  feature = "with-diesel-sqlite",
))]
mod db_migration_diesel;
#[cfg(any(
  feature = "with-sqlx-mssql",
  feature = "with-sqlx-mysql",
  feature = "with-sqlx-postgres",
  feature = "with-sqlx-sqlite",
))]
mod db_migration_sqlx;

use crate::{MigrationCommon, MigrationGroup, MigrationParams};
use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
use core::fmt;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DbMigration {
  common: MigrationCommon,
  created_on: DateTime<FixedOffset>,
  group: MigrationGroup,
}

impl DbMigration {
  #[inline]
  pub fn checksum(&self) -> &str {
    &self.common.checksum
  }

  #[inline]
  pub fn created_on(&self) -> &DateTime<FixedOffset> {
    &self.created_on
  }

  #[inline]
  pub fn group(&self) -> &MigrationGroup {
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

impl MigrationParams for DbMigration {
  #[inline]
  fn common(&self) -> &MigrationCommon {
    &self.common
  }
}

impl fmt::Display for DbMigration {
  #[inline]
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(fmt, "{}__{}", self.common.version, self.common.name)
  }
}

#[cfg(feature = "with-mysql_async")]
impl core::convert::TryFrom<mysql_async::Row> for DbMigration {
  type Error = crate::Error;

  #[inline]
  fn try_from(from: mysql_async::Row) -> Result<Self, Self::Error> {
    let self_opt = || {
      Some(Self {
        common: MigrationCommon {
          checksum: from.get("checksum")?,
          name: from.get("name")?,
          version: from.get("version")?,
        },
        created_on: _fixed_from_naive_utc(from.get::<NaiveDateTime, _>("created_on")?),
        group: MigrationGroup { version: from.get("omg_version")?, name: from.get("omg_name")? },
      })
    };
    self_opt().ok_or(crate::Error::MysqlAsync(mysql_async::Error::Driver(
      mysql_async::DriverError::FromRow { row: from },
    )))
  }
}

#[cfg(feature = "with-rusqlite")]
impl<'a> core::convert::TryFrom<&'a rusqlite::Row<'a>> for DbMigration {
  type Error = crate::Error;

  #[inline]
  fn try_from(from: &'a rusqlite::Row<'a>) -> Result<Self, Self::Error> {
    Ok(Self {
      common: MigrationCommon {
        checksum: from.get("checksum")?,
        name: from.get("name")?,
        version: from.get("version")?,
      },
      created_on: from.get::<_, DateTime<Utc>>("created_on")?.into(),
      group: MigrationGroup { version: from.get("omg_version")?, name: from.get("omg_name")? },
    })
  }
}

#[cfg(feature = "with-tiberius")]
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
      common: MigrationCommon {
        checksum: translate!(from.try_get::<&str, _>("checksum")).into(),
        name: translate!(from.try_get::<&str, _>("name")).into(),
        version: translate!(from.try_get("version")),
      },
      created_on: {
        let s = translate!(from.try_get::<&str, _>("created_on"));
        _mssql_date_hack(s)?
      },
      group: MigrationGroup {
        version: translate!(from.try_get("omg_version")),
        name: translate!(from.try_get::<&str, _>("omg_name")).into(),
      },
    })
  }
}

#[cfg(feature = "with-tokio-postgres")]
impl core::convert::TryFrom<tokio_postgres::Row> for DbMigration {
  type Error = crate::Error;

  #[inline]
  fn try_from(from: tokio_postgres::Row) -> Result<Self, Self::Error> {
    Ok(Self {
      common: MigrationCommon {
        checksum: from.try_get("checksum")?,
        name: from.try_get("name")?,
        version: from.try_get("version")?,
      },
      created_on: from.try_get("created_on")?,
      group: MigrationGroup {
        version: from.try_get("omg_version")?,
        name: from.try_get("omg_name")?,
      },
    })
  }
}

fn _fixed_from_naive_utc(naive: NaiveDateTime) -> DateTime<FixedOffset> {
  chrono::DateTime::<Utc>::from_utc(naive, Utc).into()
}

fn _mssql_date_hack(s: &str) -> crate::Result<DateTime<FixedOffset>> {
  let naive_rslt = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S");
  let naive = naive_rslt.map_err(|_| crate::Error::Other("Invalid date for mssql"))?;
  Ok(_fixed_from_naive_utc(naive))
}
