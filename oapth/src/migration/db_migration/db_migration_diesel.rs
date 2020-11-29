macro_rules! create_schema {
  ($mod_name:ident, $timestamp:ident) => {
    mod $mod_name {

      diesel::table! {
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

      diesel::table! {
        _oapth_migration_group (version) {
          version -> Integer,
          name -> Text,
        }
      }
    }
  };
}

#[oapth_macros::diesel_pg_]
create_schema!(schema_pg, Timestamptz);
#[oapth_macros::diesel_minus_pg_]
create_schema!(schema, Timestamp);

#[oapth_macros::diesel_minus_pg_]
use self::schema::{_oapth_migration as m, _oapth_migration_group as mg};
#[oapth_macros::diesel_pg_]
use {
  chrono::{DateTime, Utc},
  schema_pg::{_oapth_migration as m_pg, _oapth_migration_group as mg_pg},
};
use crate::{DbMigration, Database, MigrationCommon, MigrationGroup};
use diesel::{
  deserialize::{FromSql, QueryableByName},
  dsl::SqlTypeOf,
  row::NamedRow,
};

#[oapth_macros::diesel_mysql_]
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
  fn build<'a>(row: &impl NamedRow<'a, diesel::mysql::Mysql>) -> diesel::deserialize::Result<Self> {
    Ok(Self {
      common: MigrationCommon {
        checksum: NamedRow::get::<SqlTypeOf<m::checksum>, String>(row, "checksum")?,
        name: NamedRow::get::<SqlTypeOf<m::name>, String>(row, "name")?,
        version: NamedRow::get::<SqlTypeOf<m::version>, i32>(row, "version")?,
      },
      created_on: {
        let naive =
          NamedRow::get::<SqlTypeOf<m::created_on>, chrono::NaiveDateTime>(row, "created_on")?;
        crate::migration::db_migration::_fixed_from_naive_utc(naive)
      },
      db: Database::Mysql,
      group: MigrationGroup {
        name: NamedRow::get::<SqlTypeOf<mg::name>, String>(row, "omg_name")?,
        version: NamedRow::get::<SqlTypeOf<mg::version>, i32>(row, "omg_version")?,
      },
    })
  }
}

#[oapth_macros::diesel_pg_]
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
  fn build<'a>(row: &impl NamedRow<'a, diesel::pg::Pg>) -> diesel::deserialize::Result<Self> {
    Ok(Self {
      common: MigrationCommon {
        checksum: NamedRow::get::<SqlTypeOf<m_pg::checksum>, String>(row, "checksum")?,
        name: NamedRow::get::<SqlTypeOf<m_pg::name>, String>(row, "name")?,
        version: NamedRow::get::<SqlTypeOf<m_pg::version>, i32>(row, "version")?,
      },
      created_on: NamedRow::get::<SqlTypeOf<m_pg::created_on>, DateTime<Utc>>(row, "created_on")?
        .into(),
      db: Database::Pg,
      group: MigrationGroup {
        name: NamedRow::get::<SqlTypeOf<mg_pg::name>, String>(row, "omg_name")?,
        version: NamedRow::get::<SqlTypeOf<mg_pg::version>, i32>(row, "omg_version")?,
      },
    })
  }
}

#[oapth_macros::diesel_sqlite_]
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
  fn build<'a>(
    row: &impl NamedRow<'a, diesel::sqlite::Sqlite>,
  ) -> diesel::deserialize::Result<Self> {
    Ok(Self {
      common: MigrationCommon {
        checksum: NamedRow::get::<SqlTypeOf<m::checksum>, String>(row, "checksum")?,
        name: NamedRow::get::<SqlTypeOf<m::name>, String>(row, "name")?,
        version: NamedRow::get::<SqlTypeOf<m::version>, i32>(row, "version")?,
      },
      created_on: {
        let naive =
          NamedRow::get::<SqlTypeOf<m::created_on>, chrono::NaiveDateTime>(row, "created_on")?;
        crate::migration::db_migration::_fixed_from_naive_utc(naive)
      },
      db: Database::Sqlite,
      group: MigrationGroup {
        name: NamedRow::get::<SqlTypeOf<mg::name>, String>(row, "omg_name")?,
        version: NamedRow::get::<SqlTypeOf<mg::version>, i32>(row, "omg_version")?,
      },
    })
  }
}