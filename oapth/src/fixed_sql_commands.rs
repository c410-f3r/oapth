macro_rules! oapth_migration_columns {
  () => {
    "_oapth_migration_omg_version INT NOT NULL, \

    checksum VARCHAR(20) NOT NULL, \
    name VARCHAR(128) NOT NULL, \
    repeatability INTEGER NULL, \
    version INT NOT NULL, \

    CONSTRAINT _oapth_migration_unq UNIQUE (version, _oapth_migration_omg_version)"
  };
}

macro_rules! oapth_migration_group_columns {
  () => {
    "version INT NOT NULL PRIMARY KEY, \

    name VARCHAR(128) NOT NULL"
  };
}

#[oapth_macros::_mysql_or_pg]
macro_rules! serial_id {
  () => {
    "id SERIAL NOT NULL PRIMARY KEY,"
  };
}

oapth_macros::_mssql_! { pub(crate) mod mssql; }
oapth_macros::_mysql_! { pub(crate) mod mysql; }
oapth_macros::_pg_! { pub(crate) mod pg; }
oapth_macros::_sqlite_! { pub(crate) mod sqlite; }

use crate::{BackEnd, MigrationRef,MigrationGroupRef};
use arrayvec::ArrayString;
use core::fmt::Write;

#[inline]
pub(crate) async fn delete_migrations<B>(
  back_end: &mut B,
  mg: MigrationGroupRef<'_>,
  schema_prefix: &str,
  version: i32,
) -> crate::Result<()>
where
  B: BackEnd,
{
  let mut buffer = ArrayString::<[u8; 128]>::new();
  buffer.write_fmt(format_args!(
    "DELETE FROM {schema_prefix}_oapth_migration WHERE _oapth_migration_omg_version = {mg_version} AND version > {m_version}",
    m_version = version,
    mg_version = mg.version(),
    schema_prefix = schema_prefix,
  ))?;
  back_end.execute(&buffer).await?;
  Ok(())
}

#[inline]
pub(crate) async fn insert_migrations<'a, 'b, B, I>(
  back_end: &mut B,
  mg: MigrationGroupRef<'_>,
  schema_prefix: &str,
  migrations: I,
) -> crate::Result<()>
where
  B: BackEnd,
  I: Clone + Iterator<Item = MigrationRef<'a, 'a>> + 'b,
{
  let mut insert_migration_group_str = ArrayString::<[u8; 512]>::new();
  insert_migration_group_str.write_fmt(format_args!(
    "INSERT INTO {schema_prefix}_oapth_migration_group (version, name)
    SELECT * FROM (SELECT {mg_version} AS version, '{mg_name}' AS name) AS tmp
    WHERE NOT EXISTS (
      SELECT 1 FROM {schema_prefix}_oapth_migration_group WHERE version = {mg_version}
    );",
    mg_name = mg.name(),
    mg_version = mg.version(),
    schema_prefix = schema_prefix
  ))?;
  back_end.execute(&insert_migration_group_str).await?;

  back_end.transaction(migrations.clone().map(|m| m.sql_up)).await?;
  let f = |m: MigrationRef<'a, 'a>| {
    let mut buffer = ArrayString::<[u8; 512]>::new();
    buffer.write_fmt(format_args!(
      "INSERT INTO {schema_prefix}_oapth_migration (
        version, _oapth_migration_omg_version, checksum, name
      ) VALUES (
        {m_version}, {mg_version}, '{m_checksum}', '{m_name}'
      );",
      m_checksum = m.checksum(),
      m_name = m.name(),
      m_version = m.version(),
      mg_version = mg.version(),
      schema_prefix = schema_prefix,
    )).ok()?;
    Some(buffer)
  };
  back_end.transaction(migrations.filter_map(f)).await?;

  Ok(())
}

#[inline]
pub(crate) fn migrations_by_mg_version_query(
  mg_version: i32,
  schema_prefix: &str,
) -> crate::Result<ArrayString<[u8; 512]>> {
  let mut s = ArrayString::new();
  s.write_fmt(format_args!(
    "SELECT \
      _oapth_migration.version, \
      _oapth_migration_group.version as omg_version, \

      _oapth_migration_group.name as omg_name, \
      _oapth_migration.checksum, \
      _oapth_migration.created_on, \
      _oapth_migration.name, \
      _oapth_migration.repeatability \
    FROM \
      {schema_prefix}_oapth_migration_group \
    JOIN \
      {schema_prefix}_oapth_migration ON _oapth_migration._oapth_migration_omg_version = _oapth_migration_group.version \
    WHERE \
      _oapth_migration_group.version = {mg_version} \
    ORDER BY \
      _oapth_migration.version ASC;",
    mg_version = mg_version,
    schema_prefix = schema_prefix
  ))?;
  Ok(s)
}

