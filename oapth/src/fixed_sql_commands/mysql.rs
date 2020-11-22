use arrayvec::ArrayString;
use core::fmt::Write;

pub const _CREATE_MIGRATION_TABLES: &str = concat!(
  "CREATE TABLE IF NOT EXISTS _oapth_migration_group (",
  oapth_migration_group_columns!(),
  "); \
  
  CREATE TABLE IF NOT EXISTS _oapth_migration (",
  serial_id!(),
  "created_on TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,",
  oapth_migration_columns!(),
  ");"
);

// https://github.com/flyway/flyway/blob/master/flyway-core/src/main/java/org/flywaydb/core/internal/database/mysql/MySQLSchema.java
#[inline]
pub fn _all_tables(_: &str) -> crate::Result<ArrayString<[u8; 256]>> {
  let mut buffer = ArrayString::new();
  buffer.write_fmt(format_args!(
    "
    SELECT
      all_tables.table_name AS table_name
    FROM
      information_schema.tables AS all_tables
    WHERE
      all_tables.table_type IN ('BASE TABLE', 'SYSTEM VERSIONED')
    ",
  ))?;
  Ok(buffer)
}

// https://stackoverflow.com/questions/12403662/how-to-remove-all-mysql-tables-from-the-command-line-without-drop-database-permi/18625545#18625545
#[inline]
pub fn _clean() -> crate::Result<ArrayString<[u8; 1024]>> {
  let mut buffer = ArrayString::new();
  buffer.write_fmt(format_args!(
    "
    SET FOREIGN_KEY_CHECKS = 0;
    SET GROUP_CONCAT_MAX_LEN=32768;
    SET @tables = NULL;
    SELECT GROUP_CONCAT('`', table_name, '`') INTO @tables
      FROM information_schema.tables
      WHERE table_schema = (SELECT DATABASE());
    SELECT IFNULL(@tables,'dummy') INTO @tables;
    
    SET @tables = CONCAT('DROP TABLE IF EXISTS ', @tables);
    PREPARE stmt FROM @tables;
    EXECUTE stmt;
    DEALLOCATE PREPARE stmt;
    SET FOREIGN_KEY_CHECKS = 1;
    ",
  ))?;
  Ok(buffer)
}
