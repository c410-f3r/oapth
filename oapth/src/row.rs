macro_rules! create_and_impl_row {
  ($(($(#[$mac:meta])* $name:ident, $param:ty, $ret:ty, $dummy:expr)),* $(,)?) => {
      /// Raw database row
      pub trait Row {
        $($(#[$mac])* fn $name(&self, _elem: $param) -> crate::Result<$ret>;)*
      }

      impl Row for () {
        $(
          #[inline]
          fn $name(&self, _elem: $param) -> crate::Result<$ret> {
            Ok($dummy)
          }
        )*
      }

      #[cfg(feature = "sqlx-mysql")]
      impl Row for sqlx_mysql::MySqlRow {
        $(
          #[inline]
          fn $name(&self, _elem: $param) -> crate::Result<$ret> {
            use sqlx_core::row::Row;
            Ok(self.try_get(_elem)?)
          }
        )*
      }

      #[cfg(feature = "sqlx-postgres")]
      impl Row for sqlx_postgres::PgRow {
        $(
          #[inline]
          fn $name(&self, _elem: $param) -> crate::Result<$ret> {
            use sqlx_core::row::Row;
            Ok(self.try_get(_elem)?)
          }
        )*
      }

      #[cfg(feature = "sqlx-sqlite")]
      impl Row for sqlx_sqlite::SqliteRow {
        $(
          #[inline]
          fn $name(&self, _elem: $param) -> crate::Result<$ret> {
            use sqlx_core::row::Row;
            Ok(self.try_get(_elem)?)
          }
        )*
      }

      #[cfg(feature = "tiberius")]
      impl Row for tiberius::Row {
        $(
          #[inline]
          fn $name(&self, _elem: $param) -> crate::Result<$ret> {
            self.try_get(_elem)?.ok_or(crate::Error::InvalidSqlQuery)
          }
        )*
      }
  };
}

create_and_impl_row!(
  (
    /// Retrieves a `&str` from a column index.
    str_from_idx,
    usize,
    &str,
    ""
  ),
  (
    /// Retrieves a `&str` from a column name.
    str_from_name,
    &str,
    &str,
    ""
  ),
  (
    /// Retrieves a `i64` from a column index.
    i64_from_idx,
    usize,
    i64,
    0
  ),
  (
    /// Retrieves a `i64` from a column name.
    i64_from_name,
    &str,
    i64,
    0
  ),
);
