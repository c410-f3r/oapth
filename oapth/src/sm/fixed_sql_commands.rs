// Many commands were retrieved from the flyway project (https://github.com/flyway) so credits to
// the authors.

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

macro_rules! serial_id {
  () => {
    "id SERIAL NOT NULL PRIMARY KEY,"
  };
}

pub(crate) mod mssql;
pub(crate) mod mysql;
pub(crate) mod postgres;
pub(crate) mod sqlite;

use crate::{
  database::Database,
  sm::{MigrationGroup, UserMigration},
  DatabaseTy,
};
use alloc::{string::String, vec::Vec};
use core::fmt::Write;

#[inline]
pub(crate) async fn _delete_migrations<D, S>(
  buffer_cmd: &mut String,
  db: &mut D,
  mg: &MigrationGroup<S>,
  schema_prefix: &str,
  version: i32,
) -> crate::Result<()>
where
  D: Database,
  S: AsRef<str>,
{
  buffer_cmd.write_fmt(format_args!(
    "DELETE FROM {schema_prefix}_oapth_migration WHERE _oapth_migration_omg_version = {mg_version} AND version > {m_version}",
    m_version = version,
    mg_version = mg.version(),
    schema_prefix = schema_prefix,
  ))?;
  db.execute(buffer_cmd).await?;
  buffer_cmd.clear();
  Ok(())
}

#[inline]
pub(crate) async fn _insert_migrations<'migration, D, DBS, I, S>(
  buffer_cmd: &mut String,
  db: &mut D,
  mg: &MigrationGroup<S>,
  migrations: I,
  schema_prefix: &str,
) -> crate::Result<()>
where
  D: Database,
  DBS: AsRef<[DatabaseTy]> + 'migration,
  I: Clone + Iterator<Item = &'migration UserMigration<DBS, S>>,
  S: AsRef<str> + 'migration,
{
  buffer_cmd.write_fmt(format_args!(
    "INSERT INTO {schema_prefix}_oapth_migration_group (version, name)
    SELECT * FROM (SELECT {mg_version} AS version, '{mg_name}' AS name) AS tmp
    WHERE NOT EXISTS (
      SELECT 1 FROM {schema_prefix}_oapth_migration_group WHERE version = {mg_version}
    );",
    mg_name = mg.name(),
    mg_version = mg.version(),
    schema_prefix = schema_prefix
  ))?;
  db.execute(&*buffer_cmd).await?;
  buffer_cmd.clear();

  for migration in migrations.clone() {
    buffer_cmd.push_str(migration.sql_up());
  }
  db.transaction(&*buffer_cmd).await?;
  buffer_cmd.clear();

  for migration in migrations {
    buffer_cmd.write_fmt(format_args!(
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
  }
  db.transaction(&*buffer_cmd).await?;
  buffer_cmd.clear();

  Ok(())
}

#[inline]
pub(crate) async fn _migrations_by_mg_version_query<E, D>(
  buffer_cmd: &mut String,
  db: &mut D,
  mg_version: i32,
  results: &mut Vec<crate::sm::DbMigration>,
  schema_prefix: &str,
) -> Result<(), E>
where
  D: Database,
  E: From<crate::Error>,
  crate::sm::DbMigration: crate::FromRow<D::Row, Error = E>,
{
  buffer_cmd.write_fmt(format_args!(
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
  )).map_err(From::from)?;
  db.simple_entities(buffer_cmd, results).await?;
  buffer_cmd.clear();
  Ok(())
}
