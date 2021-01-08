use arrayvec::ArrayString;
use core::fmt::Write;

pub(crate) const CREATE_MIGRATION_TABLES: &str = concat!(
  "CREATE TABLE IF NOT EXISTS _oapth_migration_group (",
  oapth_migration_group_columns!(),
  "); \
  
  CREATE TABLE IF NOT EXISTS _oapth_migration (",
  serial_id!(),
  "created_on TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,",
  oapth_migration_columns!(),
  ");"
);

#[oapth_macros::_dev_tools]
#[inline]
pub(crate) async fn clean<B>(back_end: &mut B) -> crate::Result<()>
where
  B: crate::BackEnd,
{
  let mut buffer: ArrayString<[u8; 1024]> = ArrayString::new();

  buffer.write_fmt(format_args!("SET FOREIGN_KEY_CHECKS = 0;"))?;

  for table in back_end.tables("").await? {
    buffer.write_fmt(format_args!("DROP TABLE {} CASCADE;", table))?;
  }

  buffer.write_fmt(format_args!("SET FOREIGN_KEY_CHECKS = 1;"))?;

  back_end.execute(&buffer).await?;

  Ok(())
}

// https://github.com/flyway/flyway/blob/master/flyway-core/src/main/java/org/flywaydb/core/internal/database/mysql/MySQLSchema.java
#[inline]
pub(crate) fn tables(_: &str) -> crate::Result<ArrayString<[u8; 256]>> {
  let mut buffer = ArrayString::new();
  buffer.write_fmt(format_args!(
    "
    SELECT
      table_name AS generic_column
    FROM
      information_schema.tables
    WHERE
      tables.table_type IN ('BASE TABLE', 'SYSTEM VERSIONED')
    ",
  ))?;
  Ok(buffer)
}