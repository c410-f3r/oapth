use crate::{
  database::Database,
  orm::{
    buffer_write_fmt, seek_related_entities, write_select_field, InitialInsertValue, SelectLimit,
    SelectOrderBy, SqlWriter, Table, TableParams,
  },
  FromRows,
};
use alloc::{string::String, vec::Vec};
use core::future::Future;

/// Create, read, update and delete entities.
pub trait Crud: Database {
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
  fn read_all<'entity, T>(
    &mut self,
    (buffer_cmd, buffer_rows): (&mut String, &mut Vec<Self::Row>),
    results: &mut Vec<T>,
    tp: &TableParams<'entity, T>,
  ) -> impl Future<Output = Result<(), <T as Table<'entity>>::Error>>
  where
    T: FromRows<Error = <T as Table<'entity>>::Error, Row = Self::Row> + Table<'entity>,
    T::Associations: SqlWriter<Error = <T as Table<'entity>>::Error>,
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
  fn read_all_with_params<'entity, T>(
    &mut self,
    (buffer_cmd, buffer_rows): (&mut String, &mut Vec<Self::Row>),
    order_by: SelectOrderBy,
    results: &mut Vec<T>,
    select_limit: SelectLimit,
    tp: &TableParams<'entity, T>,
    where_str: &str,
  ) -> impl Future<Output = Result<(), <T as Table<'entity>>::Error>>
  where
    T: FromRows<Error = <T as Table<'entity>>::Error, Row = Self::Row> + Table<'entity>,
    T::Associations: SqlWriter<Error = <T as Table<'entity>>::Error>,
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
  fn read_by_id<'entity, T>(
    &mut self,
    buffer_cmd: &mut String,
    id: &T::PrimaryKeyValue,
    tp: &TableParams<'entity, T>,
  ) -> impl Future<Output = Result<T, <T as Table<'entity>>::Error>>
  where
    T: FromRows<Error = <T as Table<'entity>>::Error, Row = Self::Row> + Table<'entity>,
    T::Associations: SqlWriter<Error = <T as Table<'entity>>::Error>,
  {
    async move {
      tp.write_select(&mut *buffer_cmd, SelectOrderBy::Ascending, SelectLimit::All, &mut |b| {
        write_select_field(
          b,
          T::TABLE_NAME,
          T::TABLE_NAME_ALIAS,
          tp.table_suffix(),
          tp.id_field().name(),
        )?;
        buffer_write_fmt(b, format_args!(" = {id}"))
      })?;
      let row = self.row(&mut *buffer_cmd).await?;
      buffer_cmd.clear();
      Ok(T::from_rows(&mut *buffer_cmd, &row, &[], tp.table_suffix())?.1)
    }
  }
}

impl<T> Crud for T where T: Database {}

/// Collects all entities composed by all different rows.
///
/// One entity can constructed by more than one row.
#[inline]
fn collect_entities_tables<'entity, T>(
  (buffer_cmd, buffer_rows): (&mut String, &mut Vec<T::Row>),
  results: &mut Vec<T>,
  tp: &TableParams<'entity, T>,
) -> Result<(), <T as Table<'entity>>::Error>
where
  T: FromRows<Error = <T as Table<'entity>>::Error> + Table<'entity>,
{
  let mut counter: usize = 0;

  loop {
    if counter >= buffer_rows.len() {
      break;
    }
    let actual_rows = buffer_rows.get(counter..).unwrap_or_default();
    let suffix = tp.table_suffix();
    let skip = seek_related_entities(buffer_cmd, actual_rows, suffix, suffix, |entitiy| {
      results.push(entitiy);
      Ok(())
    })?;
    counter = counter.wrapping_add(skip);
  }

  Ok(())
}
