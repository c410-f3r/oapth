use crate::{DbMigration, Database, MigrationCommon, MigrationGroup};

#[oapth_macros::sqlx_mssql_]
impl core::convert::TryFrom<sqlx_core::mssql::MssqlRow> for DbMigration {
  type Error = crate::Error;

  #[inline]
  fn try_from(from: sqlx_core::mssql::MssqlRow) -> Result<Self, Self::Error> {
    use sqlx_core::row::Row;
    Ok(Self {
      common: MigrationCommon {
        checksum: from.try_get("checksum")?,
        name: from.try_get("name")?,
        version: from.try_get("version")?,
      },
      created_on: {
        let s = from.try_get::<String, _>("created_on")?;
        crate::migration::db_migration::mssql_date_hack(&s)?
      },
      db: Database::Mssql,
      group: MigrationGroup {
        version: from.try_get("omg_version")?,
        name: from.try_get("omg_name")?,
      },
    })
  }
}

#[oapth_macros::sqlx_mysql_]
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
      created_on: from.try_get::<chrono::DateTime<chrono::Utc>, _>("created_on")?.into(),
      db: Database::Mysql,
      group: MigrationGroup {
        version: from.try_get("omg_version")?,
        name: from.try_get("omg_name")?,
      },
    })
  }
}

#[oapth_macros::sqlx_pg_]
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
      db: Database::Pg,
      group: MigrationGroup {
        version: from.try_get("omg_version")?,
        name: from.try_get("omg_name")?,
      },
    })
  }
}

#[oapth_macros::sqlx_sqlite_]
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
      db: Database::Sqlite,
      group: MigrationGroup {
        version: from.try_get("omg_version")?,
        name: from.try_get("omg_name")?,
      },
    })
  }
}