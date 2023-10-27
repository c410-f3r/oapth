use crate::{sm::SchemaManagement, Identifier};
use alloc::{string::String, vec::Vec};
use core::fmt::Write;

pub(crate) const _CREATE_MIGRATION_TABLES: &str = concat!(
  "CREATE TABLE IF NOT EXISTS _oapth_migration_group (",
  oapth_migration_group_columns!(),
  "); \
  CREATE TABLE IF NOT EXISTS _oapth_migration (",
  serial_id!(),
  "created_on TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,",
  oapth_migration_columns!(),
  ");"
);

#[inline]
pub(crate) async fn _clear<D>(
  (buffer_cmd, buffer_idents): (&mut String, &mut Vec<Identifier>),
  db: &mut D,
) -> crate::Result<()>
where
  D: SchemaManagement,
{
  db.table_names(buffer_cmd, buffer_idents, "").await?;
  buffer_cmd.push_str("SET FOREIGN_KEY_CHECKS = 0;");
  for table in &*buffer_idents {
    buffer_cmd.write_fmt(format_args!("DROP TABLE {table} CASCADE;"))?;
  }
  buffer_cmd.push_str("SET FOREIGN_KEY_CHECKS = 1;");
  db.execute(buffer_cmd).await?;
  buffer_idents.clear();
  buffer_cmd.clear();
  Ok(())
}

#[inline]
pub(crate) async fn _table_names<D>(
  buffer_cmd: &mut String,
  db: &mut D,
  results: &mut Vec<Identifier>,
  schema: &str,
) -> crate::Result<()>
where
  D: SchemaManagement,
{
  buffer_cmd.write_fmt(format_args!(
    "
  SELECT
    table_name AS generic_column
  FROM
    information_schema.tables
  WHERE
    table_schema='{schema}' && tables.table_type IN ('BASE TABLE', 'SYSTEM VERSIONED');"
  ))?;
  db.simple_entities(buffer_cmd, results).await?;
  buffer_cmd.clear();
  Ok(())
}
