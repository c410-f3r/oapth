use crate::orm::{buffer_write_fmt, GenericSqlValue};
use alloc::string::String;

/// Raw SQL representation of a type
pub trait SqlValue {
  /// See [crate::Error].
  type Error: From<crate::Error>;

  /// Pushes the representation into `buffer_cmd`.
  fn write(&self, buffer_cmd: &mut String) -> Result<(), Self::Error>;
}

impl<T> SqlValue for &'_ T
where
  T: SqlValue,
{
  type Error = T::Error;

  #[inline]
  fn write(&self, buffer_cmd: &mut String) -> Result<(), Self::Error> {
    (**self).write(buffer_cmd)
  }
}

impl<T> SqlValue for Option<T>
where
  T: SqlValue,
{
  type Error = T::Error;

  #[inline]
  fn write(&self, buffer_cmd: &mut String) -> Result<(), Self::Error> {
    if let Some(ref elem) = *self {
      elem.write(buffer_cmd)
    } else {
      buffer_cmd.push_str("null");
      Ok(())
    }
  }
}

macro_rules! impl_display {
  ($ty:ty $(, $($bounds:tt)+)?) => {
    impl<E, $($($bounds)+)?> SqlValue for GenericSqlValue<E, $ty>
    where
      E: From<crate::Error>
    {
      type Error = E;

      #[inline]
      fn write(&self, buffer_cmd: &mut String) -> Result<(), Self::Error> {
        buffer_write_fmt(buffer_cmd, format_args!("'{}'", self.elem()))
      }
    }

    impl<E, $($($bounds)+)?> SqlValue for GenericSqlValue<E, &$ty>
    where
      E: From<crate::Error>
    {
      type Error = E;

      #[inline]
      fn write(&self, buffer_cmd: &mut String) -> Result<(), Self::Error> {
        buffer_write_fmt(buffer_cmd, format_args!("'{}'", self.elem()))
      }
    }
  }
}

impl_display!(&'_ str);
impl_display!(bool);
impl_display!(i32);
impl_display!(i64);
impl_display!(u32);
impl_display!(u64);
impl_display!(String);

#[cfg(feature = "arrayvec")]
impl_display!(arrayvec::ArrayString<N>, const N: usize);
#[cfg(feature = "rust_decimal")]
impl_display!(rust_decimal::Decimal);
