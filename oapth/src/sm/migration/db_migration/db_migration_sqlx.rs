#[cfg(feature = "sqlx-mysql")]
impl crate::FromRow<sqlx_mysql::MySqlRow> for crate::sm::DbMigration {
  type Error = crate::Error;

  #[inline]
  fn from_row(from: &sqlx_mysql::MySqlRow) -> Result<Self, Self::Error> {
    use sqlx_core::row::Row;
    Ok(Self {
      common: crate::sm::MigrationCommon {
        checksum: crate::sm::migration::db_migration::_checksum_from_str(
          from.try_get("checksum")?,
        )?,
        name: from.try_get::<&str, _>("name")?.try_into()?,
        repeatability: crate::sm::migration::db_migration::_from_opt_i32(
          from.try_get("repeatability")?,
        ),
        version: from.try_get("version")?,
      },
      created_on: from.try_get::<chrono::DateTime<chrono::Utc>, _>("created_on")?.into(),
      db_ty: crate::DatabaseTy::MySql,
      group: crate::sm::MigrationGroup::new(
        from.try_get::<&str, _>("omg_name")?.try_into()?,
        from.try_get("omg_version")?,
      ),
    })
  }
}

#[cfg(feature = "sqlx-postgres")]
impl crate::FromRow<sqlx_postgres::PgRow> for crate::sm::DbMigration {
  type Error = crate::Error;

  #[inline]
  fn from_row(from: &sqlx_postgres::PgRow) -> Result<Self, Self::Error> {
    use sqlx_core::row::Row;
    Ok(Self {
      common: crate::sm::MigrationCommon {
        checksum: crate::sm::migration::db_migration::_checksum_from_str(
          from.try_get("checksum")?,
        )?,
        name: from.try_get::<&str, _>("name")?.try_into()?,
        repeatability: crate::sm::migration::db_migration::_from_opt_i32(
          from.try_get("repeatability")?,
        ),
        version: from.try_get("version")?,
      },
      created_on: from.try_get("created_on")?,
      db_ty: crate::DatabaseTy::Postgres,
      group: crate::sm::MigrationGroup::new(
        from.try_get::<&str, _>("omg_name")?.try_into()?,
        from.try_get("omg_version")?,
      ),
    })
  }
}

#[cfg(feature = "sqlx-sqlite")]
impl crate::FromRow<sqlx_sqlite::SqliteRow> for crate::sm::DbMigration {
  type Error = crate::Error;

  #[inline]
  fn from_row(from: &sqlx_sqlite::SqliteRow) -> Result<Self, Self::Error> {
    use sqlx_core::row::Row;
    Ok(Self {
      common: crate::sm::MigrationCommon {
        checksum: crate::sm::migration::db_migration::_checksum_from_str(
          from.try_get("checksum")?,
        )?,
        name: from.try_get::<&str, _>("name")?.try_into()?,
        repeatability: crate::sm::migration::db_migration::_from_opt_i32(
          from.try_get("repeatability")?,
        ),
        version: from.try_get("version")?,
      },
      created_on: from.try_get("created_on")?,
      db_ty: crate::DatabaseTy::Sqlite,
      group: crate::sm::MigrationGroup::new(
        from.try_get::<&str, _>("omg_name")?.try_into()?,
        from.try_get("omg_version")?,
      ),
    })
  }
}
