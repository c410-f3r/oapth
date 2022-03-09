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

use oapth_commons::Database;

use crate::{Backend, Migration, MigrationGroup};
use core::fmt::Write;

#[inline]
pub(crate) async fn delete_migrations<B, S>(
  backend: &mut B,
  buffer: &mut String,
  mg: &MigrationGroup<S>,
  schema_prefix: &str,
  version: i32,
) -> crate::Result<()>
where
  B: Backend,
  S: AsRef<str>
{
  buffer.write_fmt(format_args!(
    "DELETE FROM {schema_prefix}_oapth_migration WHERE _oapth_migration_omg_version = {mg_version} AND version > {m_version}",
    m_version = version,
    mg_version = mg.version(),
    schema_prefix = schema_prefix,
  ))?;
  backend.execute(buffer).await?;
  buffer.clear();
  Ok(())
}

#[inline]
pub(crate) async fn insert_migrations<'migration, B, DBS, I, S>(
  backend: &mut B,
  buffer: &mut String,
  mg: &MigrationGroup<S>,
  schema_prefix: &str,
  migrations: I,
) -> crate::Result<()>
where
  B: Backend,
  DBS: AsRef<[Database]> + 'migration,
  I: Clone + Iterator<Item = &'migration Migration<DBS, S>> + Send,
  S: AsRef<str> + Send + Sync + 'migration
{
  buffer.write_fmt(format_args!(
    "INSERT INTO {schema_prefix}_oapth_migration_group (version, name)
    SELECT * FROM (SELECT {mg_version} AS version, '{mg_name}' AS name) AS tmp
    WHERE NOT EXISTS (
      SELECT 1 FROM {schema_prefix}_oapth_migration_group WHERE version = {mg_version}
    );",
    mg_name = mg.name(),
    mg_version = mg.version(),
    schema_prefix = schema_prefix
  ))?;
  backend.execute(buffer).await?;
  buffer.clear();

  backend.transaction(migrations.clone().map(|m| &m.sql_up)).await?;

  for migration in migrations {
    buffer.write_fmt(format_args!(
      "INSERT INTO {schema_prefix}_oapth_migration (
        version, _oapth_migration_omg_version, checksum, name
      ) VALUES (
        {m_version}, {mg_version}, '{m_checksum}', '{m_name}'
      );",
      m_checksum = migration.checksum(),
      m_name = migration.name(),
      m_version = migration.version(),
      mg_version = mg.version(),
      schema_prefix = schema_prefix,
    ))?;
  };
  
  backend.transaction([&mut *buffer].into_iter()).await?;
  buffer.clear();

  Ok(())
}

#[inline]
pub(crate) fn migrations_by_mg_version_query(
  buffer: &mut String,
  mg_version: i32,
  schema_prefix: &str,
) -> crate::Result<()> {
  buffer.write_fmt(format_args!(
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
  Ok(())
}

