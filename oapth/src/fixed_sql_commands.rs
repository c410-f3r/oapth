macro_rules! oapth_migration_columns {
  () => {
    "_oapth_migration_omg_version INT NOT NULL, \

    checksum VARCHAR(20) NOT NULL, \
    name VARCHAR(128) NOT NULL, \
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

macro_rules! serial_id {
  () => {
    "id SERIAL NOT NULL PRIMARY KEY,"
  };
}

pub(crate) mod mssql;
pub(crate) mod mysql;
pub(crate) mod postgres;
pub(crate) mod sqlite;

use crate::{BackEnd, Migration};
use arrayvec::ArrayString;
use core::fmt::Write;

#[inline]
pub async fn _delete_migrations<B>(
  back_end: &mut B,
  mg: &crate::MigrationGroup,
  schema: &str,
  version: i32,
) -> crate::Result<()>
where
  B: BackEnd,
{
  let mut buffer = ArrayString::<[u8; 128]>::new();
  buffer.write_fmt(format_args!(
    "DELETE FROM {schema}_oapth_migration WHERE _oapth_migration_omg_version = {mg_version} AND version > {m_version}",
    m_version = version,
    mg_version = mg.version(),
    schema = schema,
  ))?;
  back_end.execute(&buffer).await?;
  Ok(())
}

#[inline]
pub async fn _insert_migrations<'a, B, I>(
  back_end: &'a mut B,
  mg: &'a crate::MigrationGroup,
  schema: &str,
  migrations: I,
) -> crate::Result<()>
where
  B: BackEnd,
  I: Clone + Iterator<Item = &'a Migration> + 'a,
{
  let mut insert_migration_group_str = ArrayString::<[u8; 512]>::new();
  insert_migration_group_str.write_fmt(format_args!(
    "INSERT INTO {schema}_oapth_migration_group (version, name)
    SELECT * FROM (SELECT {mg_version} AS version, '{mg_name}' AS name) AS tmp
    WHERE NOT EXISTS (
      SELECT 1 FROM {schema}_oapth_migration_group WHERE version = {mg_version}
    );",
    mg_name = mg.name(),
    mg_version = mg.version(),
    schema = schema
  ))?;
  back_end.execute(&insert_migration_group_str).await?;

  back_end.transaction(migrations.clone().map(|m| m.sql_up())).await?;
  back_end
    .transaction(
      migrations.filter_map(|m| _insert_oapth_migration_str(mg.version(), &m.common, schema).ok()),
    )
    .await?;

  Ok(())
}

#[inline]
pub fn _migrations_by_mg_version_query(
  mg_version: i32,
  schema: &str,
) -> crate::Result<ArrayString<[u8; 512]>> {
  let mut s = ArrayString::new();
  s.write_fmt(format_args!(
    "SELECT \
      _oapth_migration.version, \
      _oapth_migration_group.version as omg_version, \

      _oapth_migration_group.name as omg_name, \
      _oapth_migration.checksum, \
      _oapth_migration.created_on, \
      _oapth_migration.name \
    FROM \
      {schema}_oapth_migration_group \
    JOIN \
      {schema}_oapth_migration ON _oapth_migration._oapth_migration_omg_version = _oapth_migration_group.version \
    WHERE \
      _oapth_migration_group.version = {mg_version} \
    ORDER BY \
      _oapth_migration.version ASC;",
    mg_version = mg_version,
    schema = schema
  ))?;
  Ok(s)
}

#[inline]
fn _insert_oapth_migration_str(
  mg_version: i32,
  m: &crate::MigrationCommon,
  schema: &str,
) -> crate::Result<ArrayString<[u8; 512]>> {
  let mut buffer = ArrayString::<[u8; 512]>::new();
  buffer.write_fmt(format_args!(
    "INSERT INTO {schema}_oapth_migration (
      version, _oapth_migration_omg_version, checksum, name
    ) VALUES (
      {m_version}, {mg_version}, '{m_checksum}', '{m_name}'
    );",
    m_checksum = m.checksum,
    m_name = m.name,
    m_version = m.version,
    mg_version = mg_version,
    schema = schema,
  ))?;
  Ok(buffer)
}
