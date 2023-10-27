use crate::{sm::SchemaManagement, Identifier};
use alloc::{string::String, vec::Vec};
use core::fmt::Write;

pub(crate) const _CREATE_MIGRATION_TABLES: &str = concat!(
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
pub(crate) async fn _clear<D>(
  (buffer_cmd, buffer_idents): (&mut String, &mut Vec<Identifier>),
  db: &mut D,
) -> crate::Result<()>
where
  D: SchemaManagement,
{
  db.table_names(buffer_cmd, buffer_idents, "").await?;
  for table in &*buffer_idents {
    buffer_cmd.write_fmt(format_args!("DROP TABLE {table};"))?;
  }
  db.execute(buffer_cmd).await?;
  buffer_cmd.clear();
  buffer_idents.clear();
  Ok(())
}

pub(crate) async fn _table_names<D>(
  _: &mut String,
  db: &mut D,
  results: &mut Vec<Identifier>,
  _: &str,
) -> crate::Result<()>
where
  D: SchemaManagement,
{
  db.simple_entities(
    "SELECT tbl_name generic_column FROM sqlite_master tables WHERE type='table' AND tbl_name NOT LIKE 'sqlite_%';",
    results
  ).await
}
