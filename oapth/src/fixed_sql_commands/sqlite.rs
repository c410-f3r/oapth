use arrayvec::ArrayString;
use core::fmt::Write;

pub const _CREATE_MIGRATION_TABLES: &str = concat!(
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
pub fn _all_tables(_: &str) -> crate::Result<ArrayString<[u8; 128]>> {
  let mut buffer = ArrayString::new();
  buffer.write_fmt(format_args!(
    "SELECT tbl_name table_name FROM sqlite_master all_tables WHERE type='table' AND tbl_name NOT LIKE 'sqlite_%';"
  ))?;
  Ok(buffer)
}

#[inline]
pub fn _clean() -> crate::Result<ArrayString<[u8; 256]>> {
  let mut buffer = ArrayString::new();
  buffer.write_fmt(format_args!(
    "
    PRAGMA writable_schema = 1;
    DELETE FROM sqlite_master WHERE type IN ('table', 'index', 'trigger');
    PRAGMA writable_schema = 0;
    ",
  ))?;
  Ok(buffer)
}
