use core::fmt::Write;
use arrayvec::ArrayString;

pub(crate) const CREATE_MIGRATION_TABLES: &str = concat!(
  "CREATE TABLE IF NOT EXISTS _oapth_migration_group (",
  oapth_migration_group_columns!(),
  "); \
  CREATE TABLE IF NOT EXISTS _oapth_migration (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    created_on TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,",
  oapth_migration_columns!(),
  ");"
);

#[oapth_macros::_dev_tools]
#[inline]
pub(crate) async fn clean<B>(backend: &mut B, buffer: &mut String) -> crate::Result<()>
where
  B: crate::Backend,
{
  for table in backend.tables("").await? {
    buffer.write_fmt(format_args!("DROP TABLE {};", table))?;
  }

  backend.execute(buffer).await?;
  buffer.clear();

  Ok(())
}

#[inline]
pub(crate) fn tables(_: &str) -> crate::Result<ArrayString<128>> {
  let mut buffer = ArrayString::new();
  buffer.write_fmt(format_args!(
    "SELECT tbl_name generic_column FROM sqlite_master tables WHERE type='table' AND tbl_name NOT LIKE 'sqlite_%';"
  ))?;
  Ok(buffer)
}