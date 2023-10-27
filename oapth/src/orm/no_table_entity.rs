use core::marker::PhantomData;

/// An empty entity for testing purposes
#[derive(Debug)]
pub struct NoTableEntity<E>(PhantomData<E>);

impl<E> NoTableEntity<E> {
  /// Creates a new instance regardless of `E`
  #[inline]
  pub const fn new() -> Self {
    Self(PhantomData)
  }
}
