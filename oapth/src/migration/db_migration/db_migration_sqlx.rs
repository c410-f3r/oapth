use crate::{DbMigration, migration::db_migration::{checksum_from_str, from_opt_i32}, MigrationCommon, MigrationGroup};
use oapth_commons::Database;

#[oapth_macros::_sqlx_mssql]
impl TryFrom<sqlx_core::mssql::MssqlRow> for DbMigration {
  type Error = crate::Error;

  #[inline]
  fn try_from(from: sqlx_core::mssql::MssqlRow) -> Result<Self, Self::Error> {
    use sqlx_core::row::Row;
    Ok(Self {
      common: MigrationCommon {
        checksum: checksum_from_str(&from.try_get::<String, _>("checksum")?)?,
        name: from.try_get("name")?,
        repeatability: from_opt_i32(from.try_get("repeatability")?),
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

#[oapth_macros::_sqlx_mysql]
impl TryFrom<sqlx_core::mysql::MySqlRow> for DbMigration {
  type Error = crate::Error;

  #[inline]
  fn try_from(from: sqlx_core::mysql::MySqlRow) -> Result<Self, Self::Error> {
    use sqlx_core::row::Row;
    Ok(Self {
      common: MigrationCommon {
        checksum: checksum_from_str(&from.try_get::<String, _>("checksum")?)?,
        name: from.try_get("name")?,
        repeatability: from_opt_i32(from.try_get("repeatability")?),
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

#[oapth_macros::_sqlx_pg]
impl TryFrom<sqlx_core::postgres::PgRow> for DbMigration {
  type Error = crate::Error;

  #[inline]
  fn try_from(from: sqlx_core::postgres::PgRow) -> Result<Self, Self::Error> {
    use sqlx_core::row::Row;
    Ok(Self {
      common: MigrationCommon {
        checksum: checksum_from_str(&from.try_get::<String, _>("checksum")?)?,
        name: from.try_get("name")?,
        repeatability: from_opt_i32(from.try_get("repeatability")?),
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

#[oapth_macros::_sqlx_sqlite]
impl TryFrom<sqlx_core::sqlite::SqliteRow> for DbMigration {
  type Error = crate::Error;

  #[inline]
  fn try_from(from: sqlx_core::sqlite::SqliteRow) -> Result<Self, Self::Error> {
    use sqlx_core::row::Row;
    Ok(Self {
      common: MigrationCommon {
        checksum: checksum_from_str(&from.try_get::<String, _>("checksum")?)?,
        name: from.try_get("name")?,
        repeatability: from_opt_i32(from.try_get("repeatability")?),
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