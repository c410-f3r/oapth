mod fx_hasher;

use crate::{
  orm::{AuxNodes, FullTableAssociation, Table, TableParams},
  FromRows, Row, TableSuffix,
};
use alloc::string::String;
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

/// Seeks all rows that equals `T`'s primary key and suffix. Can be `T` itself or any other
/// associated/related entity.
#[inline]
pub fn seek_related_entities<'entity, E, T>(
  buffer_cmd: &mut String,
  rows: &[T::Row],
  ts: TableSuffix,
  ts_related: TableSuffix,
  mut cb: impl FnMut(T) -> Result<(), E>,
) -> Result<usize, E>
where
  E: From<crate::Error>,
  T: FromRows<Error = E> + Table<'entity, Error = E>,
{
  if rows.is_empty() {
    return Ok(0);
  }

  let first_row = if let Some(elem) = rows.first() {
    elem
  } else {
    return Ok(0);
  };

  let first_rslt = T::from_rows(buffer_cmd, first_row, rows, ts_related);
  let (mut counter, mut previous) = if let Ok((skip, entity)) = first_rslt {
    write_column_alias(buffer_cmd, T::TABLE_NAME, ts, T::PRIMARY_KEY_NAME)?;
    let previous = first_row.i64_from_name(buffer_cmd.as_ref()).map_err(Into::into)?;
    buffer_cmd.clear();
    cb(entity)?;
    (skip, previous)
  } else {
    buffer_cmd.clear();
    return Ok(1);
  };

  loop {
    if counter >= rows.len() {
      break;
    }

    let row = if let Some(elem) = rows.get(counter) {
      elem
    } else {
      break;
    };

    let curr_rows = rows.get(counter..).unwrap_or_default();
    let (skip, entity) = T::from_rows(buffer_cmd, row, curr_rows, ts_related)?;

    write_column_alias(buffer_cmd, T::TABLE_NAME, ts, T::PRIMARY_KEY_NAME)?;
    let curr = row.i64_from_name(buffer_cmd.as_ref()).map_err(Into::into)?;
    buffer_cmd.clear();
    if previous == curr {
      cb(entity)?;
      counter = counter.wrapping_add(skip);
    } else {
      break;
    }
    previous = curr;
  }

  Ok(counter)
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
