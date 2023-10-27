use crate::{
  database::Database,
  orm::{
    buffer_write_fmt, write_column_alias, write_select_field, InitialInsertValue, SelectLimit,
    SelectOrderBy, SqlWriter, Table, TableParams,
  },
  FromRows, Row, TableSuffix,
};
use core::future::Future;

/// Create, read, update and delete entities.
pub trait Crud<P>: Database {
  /// Inserts a new table record represented by `table_params`.
  fn create<'entity, T>(
    &mut self,
    buffer_cmd: &mut String,
    table: &'entity T,
    table_params: &mut TableParams<'entity, T>,
  ) -> impl Future<Output = Result<(), T::Error>>
  where
    T: Table<'entity>,
    T::Associations: SqlWriter<Error = T::Error>,
  {
    async move {
      table_params.update_all_table_fields(table);
      table_params.write_insert::<InitialInsertValue>(
        &mut <_>::default(),
        &mut *buffer_cmd,
        &mut None,
      )?;
      self.execute(&*buffer_cmd).await.map_err(Into::into)?;
      Ok(())
    }
  }

  /// Fetches all entities from the database.
  fn read_all<'entity, E, R, T>(
    &mut self,
    (buffer_cmd, buffer_rows): (&mut String, &mut Vec<Self::Row>),
    results: &mut Vec<T>,
    tp: &mut TableParams<'entity, T>,
  ) -> impl Future<Output = Result<(), E>>
  where
    E: From<crate::Error>,
    T: FromRows<Error = E, Row = Self::Row> + Table<'entity, Error = E>,
    T::Associations: SqlWriter<Error = E>,
  {
    async move {
      tp.write_select(&mut *buffer_cmd, SelectOrderBy::Ascending, SelectLimit::All, &mut |_| {
        Ok(())
      })?;
      self
        .rows(&mut *buffer_cmd, |row| {
          buffer_rows.push(row);
          Ok(())
        })
        .await?;
      buffer_cmd.clear();
      collect_entities_tables((buffer_cmd, buffer_rows), results, tp)?;
      Ok(())
    }
  }

  /// Similar to `read_all` but expects more fine grained parameters.
  fn read_all_with_params<'entity, E, R, T>(
    &mut self,
    (buffer_cmd, buffer_rows): (&mut String, &mut Vec<Self::Row>),
    order_by: SelectOrderBy,
    results: &mut Vec<T>,
    select_limit: SelectLimit,
    tp: &mut TableParams<'entity, T>,
    where_str: &str,
  ) -> impl Future<Output = Result<(), E>>
  where
    E: From<crate::Error>,
    T: FromRows<Error = E, Row = Self::Row> + Table<'entity, Error = E>,
    T::Associations: SqlWriter<Error = E>,
  {
    async move {
      tp.write_select(&mut *buffer_cmd, order_by, select_limit, &mut |b| {
        b.push_str(where_str);
        Ok(())
      })?;
      self
        .rows(&mut *buffer_cmd, |row| {
          buffer_rows.push(row);
          Ok(())
        })
        .await?;
      buffer_cmd.clear();
      collect_entities_tables((buffer_cmd, buffer_rows), results, tp)?;
      Ok(())
    }
  }

  /// Fetches a single entity identified by `id`.
  fn read_by_id<'entity, E, R, T>(
    &mut self,
    buffer_cmd: &mut String,
    id: &T::PrimaryKeyValue,
    tp: &mut TableParams<'entity, T>,
  ) -> impl Future<Output = Result<T, E>>
  where
    E: From<crate::Error>,
    T: FromRows<Error = E, Row = Self::Row> + Table<'entity, Error = E>,
    T::Associations: SqlWriter<Error = E>,
  {
    async move {
      tp.write_select(&mut *buffer_cmd, SelectOrderBy::Ascending, SelectLimit::All, &mut |b| {
        write_select_field(
          b,
          T::TABLE_NAME,
          T::TABLE_NAME_ALIAS,
          tp.suffix(),
          tp.id_field().name(),
        )?;
        buffer_write_fmt(b, format_args!(" = {id}"))
      })?;
      let row = self.row(&mut *buffer_cmd).await?;
      buffer_cmd.clear();
      Ok(T::from_rows(&mut *buffer_cmd, &row, &[], tp.suffix())?.1)
    }
  }
}

/// Collects all entities composed by all different rows.
///
/// One entity can constructed by more than one row.
#[inline]
fn collect_entities_tables<'entity, E, T>(
  (buffer_cmd, buffer_rows): (&mut String, &mut Vec<T::Row>),
  results: &mut Vec<T>,
  tp: &mut TableParams<'entity, T>,
) -> Result<(), E>
where
  E: From<crate::Error>,
  T: FromRows<Error = E> + Table<'entity, Error = E>,
{
  let mut counter: usize = 0;

  loop {
    if counter >= buffer_rows.len() {
      break;
    }
    let actual_rows = buffer_rows.get(counter..).unwrap_or_default();
    let suffix = tp.suffix();
    let skip = seek_related_entities(buffer_cmd, actual_rows, suffix, suffix, |entitiy| {
      results.push(entitiy);
    })?;
    counter = counter.wrapping_add(skip);
  }

  Ok(())
}

/// Seeks all rows that equals `T`'s primary key and suffix. Can be `T` itself or any other
/// associated/related entity.
#[inline]
fn seek_related_entities<'entity, E, T>(
  buffer_cmd: &mut String,
  rows: &[T::Row],
  ts: TableSuffix,
  ts_related: TableSuffix,
  mut cb: impl FnMut(T),
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
    cb(entity);
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
      cb(entity);
      counter = counter.wrapping_add(skip);
    } else {
      break;
    }
    previous = curr;
  }

  Ok(counter)
}
