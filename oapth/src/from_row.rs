use crate::Row;
use arrayvec::ArrayString;

/// An element that can be represented from a single database row. In most cases it means
/// a database table without relationships.
pub trait FromRow<R>: Sized
where
  R: Row,
{
  /// See [crate::Error].
  type Error: From<crate::Error>;

  /// Fallible entry-point that maps the element.
  fn from_row(row: &R) -> Result<Self, Self::Error>;
}

impl<R, const N: usize> FromRow<R> for ArrayString<N>
where
  R: Row,
{
  type Error = crate::Error;

  #[inline]
  fn from_row(row: &R) -> Result<Self, Self::Error> {
    Ok(row.str_from_idx(0)?.try_into()?)
  }
}
