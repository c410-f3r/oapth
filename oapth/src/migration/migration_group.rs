use alloc::string::String;

/// Migration group - Owned
pub type MigrationGroupOwned = MigrationGroup<String>;
/// Migration group - Reference
pub type MigrationGroupRef<'s> = MigrationGroup<&'s str>;

/// A set of unique migrations
///
/// * Types
///
/// S: Sequence of characters
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct MigrationGroup<S> {
  pub(crate) name: S,
  pub(crate) version: i32,
}

impl<S> MigrationGroup<S>
where
  S: AsRef<str>,
{
  /// Creates a new instance from all necessary parameters.
  #[inline]
  pub const fn new(name: S, version: i32) -> Self {
    Self { name, version }
  }

  /// Name
  ///
  /// # Example
  ///
  /// ```rust
  /// use oapth::doc_tests::migration_group;
  /// assert_eq!(migration_group().name(), "initial");
  /// ```
  #[inline]
  pub fn name(&self) -> &str {
    self.name.as_ref()
  }

  /// Version
  ///
  /// # Example
  ///
  /// ```rust
  /// use oapth::doc_tests::migration_group;
  /// assert_eq!(migration_group().version(), 1);
  /// ```
  #[inline]
  pub fn version(&self) -> i32 {
    self.version
  }
}
