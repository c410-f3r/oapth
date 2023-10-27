use crate::{database::Database, sm::SchemaManagement, Identifier};
use alloc::{string::String, vec::Vec};
use core::fmt::Write;

pub(crate) const _CREATE_MIGRATION_TABLES: &str = concat!(
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
  created_on VARCHAR(32) NOT NULL DEFAULT CONVERT(VARCHAR(32), CURRENT_TIMESTAMP, 120),",
  oapth_migration_columns!(),
  ");
  END"
);

#[inline]
pub(crate) async fn _clear<D>(
  (buffer_cmd, buffer_idents): (&mut String, &mut Vec<Identifier>),
  db: &mut D,
) -> crate::Result<()>
where
  D: SchemaManagement,
{
  async fn drop_table<D>(
    (buffer_cmd, buffer_idents): (&mut String, &mut Vec<Identifier>),
    db: &mut D,
    schema: &str,
    start: usize,
  ) -> crate::Result<()>
  where
    D: SchemaManagement,
  {
    db.table_names(buffer_cmd, buffer_idents, schema).await?;
    for table in buffer_idents.get(start..).into_iter().flatten() {
      buffer_cmd.write_fmt(format_args!("DROP TABLE {schema}.{table};"))?;
    }
    db.execute(buffer_cmd).await?;
    buffer_cmd.clear();
    buffer_idents.truncate(start);
    Ok(())
  }

  _schemas(db, buffer_idents).await?;
  let start = buffer_idents.len();
  for _ in 0..start {
    let Some(schema) = buffer_idents.first().copied() else {
      break;
    };
    drop_table((buffer_cmd, buffer_idents), db, &schema, start).await?;
  }
  drop_table((buffer_cmd, buffer_idents), db, "dbo", start).await?;
  for schema in buffer_idents.iter_mut() {
    buffer_cmd.write_fmt(format_args!("DROP SCHEMA {schema};"))?;
  }
  db.execute(buffer_cmd).await?;
  buffer_cmd.clear();
  buffer_idents.clear();
  Ok(())
}

#[inline]
pub(crate) async fn _schemas<D>(db: &mut D, results: &mut Vec<Identifier>) -> crate::Result<()>
where
  D: Database,
{
  db.simple_entities(
    "
    SELECT
      s.name
    FROM
      sys.schemas s
      INNER JOIN sys.sysusers u ON u.uid = s.principal_id
    WHERE
      u.issqluser = 1
      AND s.name NOT IN ('dbo', 'sys', 'guest', 'INFORMATION_SCHEMA');
    ",
    results,
  )
  .await?;
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
  D: Database,
{
  buffer_cmd.write_fmt(format_args!(
    "
    SELECT
      tables.name AS generic_column
    FROM
      sys.objects AS tables LEFT JOIN sys.extended_properties AS eps ON tables.object_id = eps.major_id
      AND eps.class = 1
      AND eps.minor_id = 0
      AND eps.name='microsoft_database_tools_support'
    WHERE
      SCHEMA_NAME(tables.schema_id) = '{schema}'
      AND eps.major_id IS NULL
      AND tables.is_ms_shipped = 0
      AND tables.type IN ('U');
    "))?;
  db.simple_entities(buffer_cmd, results).await?;
  buffer_cmd.clear();
  Ok(())
}
