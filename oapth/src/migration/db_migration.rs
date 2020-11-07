use crate::{MigrationCommon, MigrationGroup, MigrationParams};
use chrono::NaiveDateTime;
use core::fmt;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DbMigration {
  pub(crate) common: MigrationCommon,
  pub(crate) created_on: NaiveDateTime,
  pub(crate) group: MigrationGroup,
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
        created_on: from.get::<NaiveDateTime, _>("created_on")?,
        group: MigrationGroup {
          version: from.get("group_version")?,
          name: from.get("group_name")?,
        },
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
      created_on: from.get("created_on")?,
      group: MigrationGroup { version: from.get("group_version")?, name: from.get("group_name")? },
    })
  }
}

#[cfg(feature = "with-sqlx-mssql")]
impl core::convert::TryFrom<sqlx_core::mssql::MssqlRow> for DbMigration {
  type Error = crate::Error;

  #[allow(clippy::panic)]
  #[inline]
  fn try_from(from: sqlx_core::mssql::MssqlRow) -> Result<Self, Self::Error> {
    use sqlx_core::row::Row;
    Ok(Self {
      common: MigrationCommon {
        checksum: from.try_get("checksum")?,
        name: from.try_get("name")?,
        version: from.try_get("version")?,
      },
      created_on: from.try_get("created_on")?,
      group: MigrationGroup {
        version: from.try_get("group_version")?,
        name: from.try_get("group_name")?,
      },
    })
  }
}

#[cfg(feature = "with-sqlx-mysql")]
impl core::convert::TryFrom<sqlx_core::mysql::MySqlRow> for DbMigration {
  type Error = crate::Error;

  #[inline]
  fn try_from(from: sqlx_core::mysql::MySqlRow) -> Result<Self, Self::Error> {
    use sqlx_core::row::Row;
    Ok(Self {
      common: MigrationCommon {
        checksum: from.try_get("checksum")?,
        name: from.try_get("name")?,
        version: from.try_get("version")?,
      },
      created_on: {
        let dt = from.try_get::<chrono::DateTime<chrono::Utc>, _>("created_on")?;
        dt.naive_utc()
      },
      group: MigrationGroup {
        version: from.try_get("group_version")?,
        name: from.try_get("group_name")?,
      },
    })
  }
}

#[cfg(feature = "with-sqlx-postgres")]
impl core::convert::TryFrom<sqlx_core::postgres::PgRow> for DbMigration {
  type Error = crate::Error;

  #[inline]
  fn try_from(from: sqlx_core::postgres::PgRow) -> Result<Self, Self::Error> {
    use sqlx_core::row::Row;
    Ok(Self {
      common: MigrationCommon {
        checksum: from.try_get("checksum")?,
        name: from.try_get("name")?,
        version: from.try_get("version")?,
      },
      created_on: from.try_get("created_on")?,
      group: MigrationGroup {
        version: from.try_get("group_version")?,
        name: from.try_get("group_name")?,
      },
    })
  }
}

#[cfg(feature = "with-sqlx-sqlite")]
impl core::convert::TryFrom<sqlx_core::sqlite::SqliteRow> for DbMigration {
  type Error = crate::Error;

  #[inline]
  fn try_from(from: sqlx_core::sqlite::SqliteRow) -> Result<Self, Self::Error> {
    use sqlx_core::row::Row;
    Ok(Self {
      common: MigrationCommon {
        checksum: from.try_get("checksum")?,
        name: from.try_get("name")?,
        version: from.try_get("version")?,
      },
      created_on: from.try_get("created_on")?,
      group: MigrationGroup {
        version: from.try_get("group_version")?,
        name: from.try_get("group_name")?,
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
        version: from.try_get("group_version")?,
        name: from.try_get("group_name")?,
      },
    })
  }
}
