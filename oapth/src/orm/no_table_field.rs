use crate::orm::TableFields;
use core::{array, marker::PhantomData};

/// For entities that don't have fields beyond the primary key
#[derive(Debug)]
pub struct NoTableField<E>(PhantomData<E>);

impl<E> NoTableField<E> {
  /// Creates a new instance regardless of `E`
  #[inline]
  pub const fn new() -> Self {
    Self(PhantomData)
  }
}

impl<E> TableFields for NoTableField<E>
where
  E: From<crate::Error>,
{
  type Error = E;
  type FieldNames = array::IntoIter<&'static str, 0>;

  #[inline]
  fn field_names(&self) -> Self::FieldNames {
    [].into_iter()
  }

  #[inline]
  fn write_insert_values(&self, _: &mut String) -> Result<(), Self::Error> {
    Ok(())
  }

  #[inline]
  fn write_update_values(&self, _: &mut String) -> Result<(), Self::Error> {
    Ok(())
  }
}
