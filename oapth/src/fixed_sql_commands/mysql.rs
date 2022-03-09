use core::fmt::Write;
  use arrayvec::ArrayString;

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
pub(crate) async fn clean<B>(backend: &mut B, buffer: &mut String) -> crate::Result<()>
where
  B: crate::Backend,
{
  buffer.write_fmt(format_args!("SET FOREIGN_KEY_CHECKS = 0;"))?;
  for table in backend.tables("").await? {
    buffer.write_fmt(format_args!("DROP TABLE {} CASCADE;", table))?;
  }
  buffer.write_fmt(format_args!("SET FOREIGN_KEY_CHECKS = 1;"))?;
  backend.execute(buffer).await?;
  buffer.clear();
  Ok(())
}

// https://github.com/flyway/flyway/blob/master/flyway-core/src/main/java/org/flywaydb/core/internal/database/mysql/MySQLSchema.java
#[inline]
pub(crate) fn tables(_: &str) -> crate::Result<ArrayString<256>> {
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