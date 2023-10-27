use core::{
  cmp::Ordering,
  fmt::{Debug, Display, Formatter},
  hash::{Hash, Hasher},
  marker::PhantomData,
};

/// Wrapper intended for generic sql types.
pub struct GenericSqlValue<E, T>(T, PhantomData<E>);

impl<E, T> GenericSqlValue<E, T> {
  /// Inner element
  #[inline]
  pub const fn elem(&self) -> &T {
    &self.0
  }
}

impl<E, T> Clone for GenericSqlValue<E, T>
where
  T: Clone,
{
  fn clone(&self) -> Self {
    Self::from(self.0.clone())
  }
}

impl<E, T> Copy for GenericSqlValue<E, T> where T: Copy {}

impl<E, T> Default for GenericSqlValue<E, T>
where
  T: Default,
{
  #[inline]
  fn default() -> Self {
    Self::from(T::default())
  }
}

impl<E, T> Debug for GenericSqlValue<E, T>
where
  T: Debug,
{
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    self.0.fmt(f)
  }
}

impl<E, T> Display for GenericSqlValue<E, T>
where
  T: Display,
{
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    self.0.fmt(f)
  }
}

impl<E, T> Eq for GenericSqlValue<E, T> where T: Eq {}

impl<E, T> From<T> for GenericSqlValue<E, T> {
  fn from(from: T) -> Self {
    Self(from, PhantomData)
  }
}

impl<E, T> Hash for GenericSqlValue<E, T>
where
  T: Hash,
{
  #[inline]
  fn hash<H>(&self, state: &mut H)
  where
    H: Hasher,
  {
    self.0.hash(state);
  }
}

impl<E, T> Ord for GenericSqlValue<E, T>
where
  T: Ord,
{
  #[inline]
  fn cmp(&self, other: &Self) -> Ordering {
    self.0.cmp(&other.0)
  }
}

impl<E, T> PartialEq for GenericSqlValue<E, T>
where
  T: PartialEq,
{
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0
  }
}

impl<E, T> PartialOrd for GenericSqlValue<E, T>
where
  T: PartialOrd,
{
  #[inline]
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.0.partial_cmp(&other.0)
  }
}
