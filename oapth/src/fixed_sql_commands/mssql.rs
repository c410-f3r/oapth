use arrayvec::ArrayString;
use core::fmt::Write;

pub const _CREATE_MIGRATION_TABLES: &str = concat!(
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
pub fn _all_tables(schema: &str) -> crate::Result<ArrayString<[u8; 512]>> {
  let mut buffer = ArrayString::new();
  buffer.write_fmt(format_args!(
    "
    SELECT
      all_tables.name AS table_name
    FROM
      sys.objects AS all_tables LEFT JOIN sys.extended_properties AS eps ON all_tables.object_id = eps.major_id
      AND eps.class = 1
      AND eps.minor_id = 0
      AND eps.name='microsoft_database_tools_support'
    WHERE
      SCHEMA_NAME(all_tables.schema_id) = '{schema}'
      AND eps.major_id IS NULL
      AND all_tables.is_ms_shipped = 0
      AND all_tables.type IN ('U');
    ",
    schema = schema
  ))?;
  Ok(buffer)
}

#[inline]
pub fn _clean() -> crate::Result<ArrayString<[u8; 1024]>> {
  let mut buffer = ArrayString::new();
  buffer.write_fmt(format_args!(
    r#"
    EXECUTE sp_msforeachtable "ALTER TABLE ? NOCHECK CONSTRAINT all"

    DECLARE @drop_tables NVARCHAR(max)='';
    SELECT
      @drop_tables += ' DROP TABLE ' + QUOTENAME(s.name) + '.'+ QUOTENAME(ifs.table_name) + '; '
    FROM
      sys.schemas s
      INNER JOIN sys.sysusers u ON u.uid = s.principal_id
      INNER JOIN information_schema.tables ifs ON ifs.table_schema = s.name
    WHERE
      u.issqluser = 1
      AND ifs.table_type = 'BASE TABLE'
      AND u.name NOT IN ('sys', 'guest', 'INFORMATION_SCHEMA');
    EXECUTE sp_executesql @drop_tables;
    
    DECLARE @drop_schemas NVARCHAR(max)='';
    SELECT
      @drop_schemas += 'DROP SCHEMA ' + QUOTENAME(s.name) + '; '
    FROM
      sys.schemas s
      INNER JOIN sys.sysusers u ON u.uid = s.principal_id
    WHERE
      u.issqluser = 1
      AND u.name NOT IN ('dbo', 'sys', 'guest', 'INFORMATION_SCHEMA');
    EXECUTE sp_executesql @drop_schemas; 

    EXECUTE sp_msforeachtable "ALTER TABLE ? WITH CHECK CHECK CONSTRAINT all"
    "#
  ))?;
  Ok(buffer)
}
