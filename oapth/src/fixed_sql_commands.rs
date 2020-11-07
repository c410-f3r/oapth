use crate::{Backend, Migration};
use arrayvec::ArrayString;
use core::{concat, fmt::Write};

macro_rules! oapth_migration_columns {
  () => {
    "_oapth_migration_group_version INT NOT NULL, \

    checksum VARCHAR(20) NOT NULL, \
    name VARCHAR(128) NOT NULL, \
    version INT NOT NULL, \

    CONSTRAINT _oapth_migration_unq UNIQUE (version, _oapth_migration_group_version)"
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

pub const _CREATE_MIGRATION_TABLES_MSSQL: &str = concat!(
  "IF (NOT EXISTS (SELECT 1 FROM sys.schemas WHERE name = '_oapth'))
  BEGIN
    EXEC ('CREATE SCHEMA [_oapth]')
  END

  IF (NOT EXISTS (
    SELECT
      1
    FROM
      information_schema.tables
    WHERE
      table_name = '_oapth_migration_group' AND table_schema = '_oapth'
  ))
  BEGIN
  CREATE TABLE _oapth._oapth_migration_group (",
  oapth_migration_group_columns!(),
  ");
  END
  
  IF (NOT EXISTS (
    SELECT
      1
    FROM
      information_schema.tables
    WHERE
      table_name = '_oapth_migration' AND table_schema = '_oapth'
  ))
  BEGIN
  CREATE TABLE _oapth._oapth_migration (
  id INT NOT NULL IDENTITY PRIMARY KEY,
  created_on TIMESTAMP NOT NULL DEFAULT CONVERT(int, CURRENT_TIMESTAMP),",
  oapth_migration_columns!(),
  ");
  END"
);

pub const _CREATE_MIGRATION_TABLES_MYSQL: &str = concat!(
  "CREATE TABLE IF NOT EXISTS _oapth_migration_group (",
  oapth_migration_group_columns!(),
  "); \

  CREATE TABLE IF NOT EXISTS _oapth_migration (",
  serial_id!(),
  "created_on TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,",
  oapth_migration_columns!(),
  ");"
);

pub const _CREATE_MIGRATION_TABLES_POSTGRESQL: &str = concat!(
  "CREATE SCHEMA IF NOT EXISTS _oapth; \

  CREATE TABLE IF NOT EXISTS _oapth._oapth_migration_group (",
  oapth_migration_group_columns!(),
  ");

  CREATE TABLE IF NOT EXISTS _oapth._oapth_migration (",
  serial_id!(),
  "created_on TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,",
  oapth_migration_columns!(),
  ");"
);

pub const _CREATE_MIGRATION_TABLES_SQLITE: &str = concat!(
  "CREATE TABLE IF NOT EXISTS _oapth_migration_group (",
  oapth_migration_group_columns!(),
  "); \

  CREATE TABLE IF NOT EXISTS _oapth_migration (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  created_on TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,",
  oapth_migration_columns!(),
  ");"
);

#[inline]
pub async fn _insert_migrations<'a, B, I>(
  backend: &'a mut B,
  mg: &'a crate::MigrationGroup,
  schema: &str,
  migrations: I,
) -> crate::Result<()>
where
  B: Backend,
  I: Clone + Iterator<Item = &'a Migration> + 'a,
{
  let _schema = if let Some(rslt) = schema.into() { rslt } else { "" };

  let mut insert_migration_group_str = ArrayString::<[u8; 512]>::new();
  insert_migration_group_str.write_fmt(format_args!(
    "INSERT INTO {schema}_oapth_migration_group (version, name)
    SELECT * FROM (SELECT {group_version} AS version, '{group_name}' AS name) AS tmp
    WHERE NOT EXISTS (
        SELECT 1 FROM {schema}_oapth_migration_group WHERE version = {group_version}
    );",
    group_version = mg.version(),
    group_name = mg.name(),
    schema = _schema
  ))?;
  backend.execute(&insert_migration_group_str).await?;

  backend.transaction(migrations.clone().map(|m| m.sql_up())).await?;
  backend
    .transaction(
      migrations.filter_map(|m| _insert_oapth_migration_str(mg.version(), &m.common, schema).ok()),
    )
    .await?;

  Ok(())
}

#[inline]
pub fn _migrations_by_group_version_query(
  group_version: i32,
  schema: &str,
) -> crate::Result<ArrayString<[u8; 512]>> {
  let mut s = ArrayString::new();
  s.write_fmt(format_args!(
    "SELECT \
      _oapth_migration_group.version as group_version, \
      _oapth_migration.version, \

      _oapth_migration_group.name as group_name, \
      _oapth_migration.checksum, \
      _oapth_migration.created_on, \
      _oapth_migration.name \
    FROM \
      {schema}_oapth_migration_group \
    JOIN \
      {schema}_oapth_migration ON _oapth_migration._oapth_migration_group_version = _oapth_migration_group.version \
    WHERE \
      _oapth_migration_group.version = {group_version} \
    ORDER BY \
      _oapth_migration.version ASC;",
    group_version = group_version,
    schema = schema
  ))?;
  Ok(s)
}

#[inline]
fn _insert_oapth_migration_str(
  group_version: i32,
  m: &crate::MigrationCommon,
  schema: &str,
) -> crate::Result<ArrayString<[u8; 512]>> {
  let mut buffer = ArrayString::<[u8; 512]>::new();
  buffer.write_fmt(format_args!(
    "INSERT INTO {schema}_oapth_migration (
      version, _oapth_migration_group_version, checksum, name
    ) VALUES (
      {m_version}, {group_version}, '{m_checksum}', '{m_name}'
    );",
    group_version = group_version,
    m_checksum = m.checksum,
    m_name = m.name,
    m_version = m.version,
    schema = schema,
  ))?;
  Ok(buffer)
}
