use core::fmt::Write;
use arrayvec::ArrayString;

pub(crate) const CREATE_MIGRATION_TABLES: &str = concat!(
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

#[oapth_macros::_dev_tools]
#[inline]
pub(crate) async fn clean<B>(backend: &mut B, buffer: &mut String) -> crate::Result<()>
where
  B: crate::Backend,
{
  let schemas = schemas(backend).await?;
  let schemas_with_dbo = schemas.iter().map(|s| s.as_str()).chain(["dbo"]);

  for schema in schemas_with_dbo {
    for table in backend.tables(schema).await? {
      buffer.write_fmt(format_args!("DROP TABLE {schema}.{};", table, schema = schema))?;
    }
    backend.execute(buffer).await?;
  }

  buffer.clear();

  for schema in schemas {
    buffer.write_fmt(format_args!("DROP SCHEMA {};", schema))?;
    backend.execute(buffer).await?;
  }

  buffer.clear();

  Ok(())
}

#[oapth_macros::_dev_tools]
#[inline]
pub(crate) async fn schemas<B>(backend: &mut B) -> crate::Result<Vec<String>>
where
  B: crate::Backend,
{
  backend.query_string("
    SELECT
      s.name
    FROM
      sys.schemas s
      INNER JOIN sys.sysusers u ON u.uid = s.principal_id
    WHERE
      u.issqluser = 1
      AND s.name NOT IN ('dbo', 'sys', 'guest', 'INFORMATION_SCHEMA');
  ").await
}

#[inline]
pub(crate) fn tables(schema: &str) -> crate::Result<ArrayString<512>> {
  let mut buffer = ArrayString::new();
  buffer.write_fmt(format_args!("
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
    ",
    schema = schema
  ))?;
  Ok(buffer)
}