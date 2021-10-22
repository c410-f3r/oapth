use arrayvec::ArrayString;
use core::fmt::Write;

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
pub(crate) async fn clean<B>(back_end: &mut B) -> crate::Result<()>
where
  B: crate::BackEnd,
{
  let schemas = schemas(back_end).await?;
  let schemas_with_dbo = schemas.iter().map(|s| s.as_str()).chain(["dbo"]);

  for schema in schemas_with_dbo {
    let mut buffer: ArrayString<[u8; 1024]> = ArrayString::new();
    
    for table in back_end.tables(schema).await? {
      buffer.write_fmt(format_args!("DROP TABLE {schema}.{};", table, schema = schema))?;
    }

    back_end.execute(&buffer).await?;
  }

  for schema in schemas {
    let mut buffer: ArrayString<[u8; 128]> = ArrayString::new();
    buffer.write_fmt(format_args!("DROP SCHEMA {};", schema))?;
    back_end.execute(&buffer).await?;
  }

  Ok(())
}

#[oapth_macros::_dev_tools]
#[inline]
pub(crate) async fn schemas<B>(back_end: &mut B) -> crate::Result<Vec<String>>
where
  B: crate::BackEnd,
{
  Ok(back_end.query_string("
    SELECT
      s.name
    FROM
      sys.schemas s
      INNER JOIN sys.sysusers u ON u.uid = s.principal_id
    WHERE
      u.issqluser = 1
      AND s.name NOT IN ('dbo', 'sys', 'guest', 'INFORMATION_SCHEMA');
  ").await?)
}

#[inline]
pub(crate) fn tables(schema: &str) -> crate::Result<ArrayString<[u8; 512]>> {
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