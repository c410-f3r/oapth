macro_rules! create_schema {
  ($mod_name:ident, $timestamp:ty) => {
    mod $mod_name {
      table! {
        _oapth_migration (id) {
          id -> Integer,

          checksum -> Text,
          created_on -> $timestamp,
          name -> Text,
          omg_name -> Text,
          omg_version -> Integer,
          version -> Integer,
        }
      }

      table! {
        _oapth_migration_group (version) {
          version -> Integer,
          name -> Text,
        }
      }
    }
  };
}

#[cfg(feature = "with-diesel-postgres")]
create_schema!(schema_pg, Timestamptz);
#[cfg(any(feature = "with-diesel-mysql", feature = "with-diesel-sqlite",))]
create_schema!(schema, Timestamp);

use crate::{DbMigration, MigrationCommon, MigrationGroup};
use diesel::{
  deserialize::{FromSql, QueryableByName},
  dsl::SqlTypeOf,
  row::NamedRow,
};
#[cfg(any(feature = "with-diesel-mysql", feature = "with-diesel-sqlite",))]
use schema::{_oapth_migration as m, _oapth_migration_group as mg};
#[cfg(feature = "with-diesel-postgres")]
use {
  chrono::{DateTime, Utc},
  schema_pg::{_oapth_migration as m_pg, _oapth_migration_group as mg_pg},
};

#[cfg(feature = "with-diesel-mysql")]
impl QueryableByName<diesel::mysql::Mysql> for DbMigration
where
  i32: FromSql<SqlTypeOf<m::omg_version>, diesel::mysql::Mysql>,
  i32: FromSql<SqlTypeOf<mg::version>, diesel::mysql::Mysql>,
  chrono::NaiveDateTime: FromSql<SqlTypeOf<m::created_on>, diesel::mysql::Mysql>,
  String: FromSql<SqlTypeOf<m::checksum>, diesel::mysql::Mysql>,
  String: FromSql<SqlTypeOf<m::name>, diesel::mysql::Mysql>,
  String: FromSql<SqlTypeOf<m::omg_name>, diesel::mysql::Mysql>,
  String: FromSql<SqlTypeOf<mg::name>, diesel::mysql::Mysql>,
{
  fn build<R>(row: &R) -> diesel::deserialize::Result<Self>
  where
    R: NamedRow<diesel::mysql::Mysql>,
  {
    Ok(Self {
      common: MigrationCommon {
        checksum: row.get::<SqlTypeOf<m::checksum>, String>("checksum")?,
        name: row.get::<SqlTypeOf<m::name>, String>("name")?,
        version: row.get::<SqlTypeOf<m::version>, i32>("version")?,
      },
      created_on: {
        let naive = row.get::<SqlTypeOf<m::created_on>, chrono::NaiveDateTime>("created_on")?;
        crate::db_migration::_fixed_from_naive_utc(naive)
      },
      group: MigrationGroup {
        name: row.get::<SqlTypeOf<mg::name>, String>("omg_name")?,
        version: row.get::<SqlTypeOf<mg::version>, i32>("omg_version")?,
      },
    })
  }
}

#[cfg(feature = "with-diesel-postgres")]
impl QueryableByName<diesel::pg::Pg> for DbMigration
where
  i32: FromSql<SqlTypeOf<m_pg::omg_version>, diesel::pg::Pg>,
  i32: FromSql<SqlTypeOf<mg_pg::version>, diesel::pg::Pg>,
  DateTime<Utc>: FromSql<SqlTypeOf<m_pg::created_on>, diesel::pg::Pg>,
  String: FromSql<SqlTypeOf<m_pg::checksum>, diesel::pg::Pg>,
  String: FromSql<SqlTypeOf<m_pg::name>, diesel::pg::Pg>,
  String: FromSql<SqlTypeOf<m_pg::omg_name>, diesel::pg::Pg>,
  String: FromSql<SqlTypeOf<mg_pg::name>, diesel::pg::Pg>,
{
  fn build<R>(row: &R) -> diesel::deserialize::Result<Self>
  where
    R: NamedRow<diesel::pg::Pg>,
  {
    Ok(Self {
      common: MigrationCommon {
        checksum: row.get::<SqlTypeOf<m_pg::checksum>, String>("checksum")?,
        name: row.get::<SqlTypeOf<m_pg::name>, String>("name")?,
        version: row.get::<SqlTypeOf<m_pg::version>, i32>("version")?,
      },
      created_on: row.get::<SqlTypeOf<m_pg::created_on>, DateTime<Utc>>("created_on")?.into(),
      group: MigrationGroup {
        name: row.get::<SqlTypeOf<mg_pg::name>, String>("omg_name")?,
        version: row.get::<SqlTypeOf<mg_pg::version>, i32>("omg_version")?,
      },
    })
  }
}

#[cfg(feature = "with-diesel-sqlite")]
impl QueryableByName<diesel::sqlite::Sqlite> for DbMigration
where
  i32: FromSql<SqlTypeOf<m::omg_version>, diesel::sqlite::Sqlite>,
  i32: FromSql<SqlTypeOf<mg::version>, diesel::sqlite::Sqlite>,
  chrono::NaiveDateTime: FromSql<SqlTypeOf<m::created_on>, diesel::sqlite::Sqlite>,
  String: FromSql<SqlTypeOf<m::checksum>, diesel::sqlite::Sqlite>,
  String: FromSql<SqlTypeOf<m::name>, diesel::sqlite::Sqlite>,
  String: FromSql<SqlTypeOf<m::omg_name>, diesel::sqlite::Sqlite>,
  String: FromSql<SqlTypeOf<mg::name>, diesel::sqlite::Sqlite>,
{
  fn build<R>(row: &R) -> diesel::deserialize::Result<Self>
  where
    R: NamedRow<diesel::sqlite::Sqlite>,
  {
    Ok(Self {
      common: MigrationCommon {
        checksum: row.get::<SqlTypeOf<m::checksum>, String>("checksum")?,
        name: row.get::<SqlTypeOf<m::name>, String>("name")?,
        version: row.get::<SqlTypeOf<m::version>, i32>("version")?,
      },
      created_on: {
        let naive = row.get::<SqlTypeOf<m::created_on>, chrono::NaiveDateTime>("created_on")?;
        crate::db_migration::_fixed_from_naive_utc(naive)
      },
      group: MigrationGroup {
        name: row.get::<SqlTypeOf<mg::name>, String>("omg_name")?,
        version: row.get::<SqlTypeOf<mg::version>, i32>("omg_version")?,
      },
    })
  }
}
