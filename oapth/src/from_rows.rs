use crate::{Row, TableSuffix};
use alloc::string::String;

/// An element that can be represented from one or more database row, in other words, tables
/// with relationships.
pub trait FromRows: Sized {
  /// Error
  type Error: From<crate::Error>;
  /// Underlying database row.
  type Row: Row;

  /// Constructs a single instance based on an arbitrary number of rows.
  fn from_rows(
    buffer_cmd: &mut String,
    curr_row: &Self::Row,
    rows: &[Self::Row],
    table_suffix: TableSuffix,
  ) -> Result<(usize, Self), Self::Error>;
}
