mod fx_hasher;

use crate::{
  orm::{AuxNodes, FullTableAssociation, Table, TableParams},
  TableSuffix,
};
use core::fmt::{Arguments, Write};
pub(crate) use fx_hasher::*;

/// Shortcut of `buffer_cmd.write_fmt(...)`
#[inline]
pub fn buffer_write_fmt<E>(buffer_cmd: &mut String, args: Arguments<'_>) -> Result<(), E>
where
  E: From<crate::Error>,
{
  buffer_cmd.write_fmt(args).map_err(|err| E::from(crate::Error::Fmt(err)))
}

/// Writes {table}{suffix}__{field}` into a buffer_cmd.
#[inline]
pub fn write_column_alias(
  buffer_cmd: &mut String,
  table: &str,
  ts: TableSuffix,
  field: &str,
) -> crate::Result<()> {
  buffer_cmd.write_fmt(format_args!("{table}{ts}__{field}",))?;
  Ok(())
}

pub(crate) fn node_was_already_visited<'entity, T>(
  aux: &mut AuxNodes,
  table: &TableParams<'entity, T>,
) -> crate::Result<bool>
where
  T: Table<'entity>,
{
  let hash = table.instance_hash();
  match aux
    .binary_search_by(|(local_hash, _)| local_hash.cmp(&hash))
    .and_then(|idx| aux.get(idx).map(|elem| elem.1).ok_or(idx))
  {
    Err(could_be_idx) => aux.insert(could_be_idx, (hash, T::TABLE_NAME)),
    Ok(existent_table_name) => {
      if existent_table_name == T::TABLE_NAME {
        return Ok(true);
      } else {
        return Err(crate::Error::HashCollision(existent_table_name, T::TABLE_NAME));
      }
    }
  }
  Ok(false)
}

#[inline]
pub(crate) fn truncate_if_ends_with_char(buffer_cmd: &mut String, c: char) {
  if buffer_cmd.ends_with(c) {
    buffer_cmd.truncate(buffer_cmd.len().wrapping_sub(1))
  }
}

#[inline]
pub(crate) fn truncate_if_ends_with_str(buffer_cmd: &mut String, s: &str) {
  if buffer_cmd.ends_with(s) {
    buffer_cmd.truncate(buffer_cmd.len().wrapping_sub(s.len()))
  }
}

#[inline]
pub(crate) fn write_full_select_field(
  buffer_cmd: &mut String,
  table: &str,
  table_alias: Option<&str>,
  ts: TableSuffix,
  field: &str,
) -> crate::Result<()> {
  let actual_table = table_alias.unwrap_or(table);
  write_select_field(buffer_cmd, table, table_alias, ts, field)?;
  buffer_cmd.write_fmt(format_args!(" AS {actual_table}{ts}__{field}"))?;
  Ok(())
}

#[inline]
pub(crate) fn write_select_field(
  buffer_cmd: &mut String,
  table: &str,
  table_alias: Option<&str>,
  ts: TableSuffix,
  field: &str,
) -> crate::Result<()> {
  let actual_table = table_alias.unwrap_or(table);
  buffer_cmd.write_fmt(format_args!("\"{actual_table}{ts}\".{field}"))?;
  Ok(())
}

#[inline]
pub(crate) fn write_select_join(
  buffer_cmd: &mut String,
  from_table: &str,
  from_table_suffix: TableSuffix,
  full_association: FullTableAssociation,
) -> crate::Result<()> {
  let association = full_association.association();
  buffer_cmd.write_fmt(format_args!(
    "LEFT JOIN \"{table_relationship}\" AS \"{table_relationship_alias}{to_table_suffix}\" ON \
     \"{from_table}{from_table_suffix}\".{table_id} = \
     \"{table_relationship_alias}{to_table_suffix}\".{table_relationship_id}",
    table_id = association.from_id(),
    table_relationship = full_association.to_table(),
    table_relationship_alias =
      full_association.to_table_alias().unwrap_or_else(|| full_association.to_table()),
    table_relationship_id = association.to_id(),
    to_table_suffix = full_association.to_table_suffix(),
  ))?;
  Ok(())
}

#[inline]
pub(crate) fn write_select_order_by(
  buffer_cmd: &mut String,
  table: &str,
  table_alias: Option<&str>,
  ts: TableSuffix,
  field: &str,
) -> crate::Result<()> {
  let actual_table = table_alias.unwrap_or(table);
  buffer_cmd.write_fmt(format_args!("\"{actual_table}{ts}\".{field}",))?;
  Ok(())
}
