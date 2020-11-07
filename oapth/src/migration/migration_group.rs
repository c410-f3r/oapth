use alloc::string::String;

/// A set of unique migrations
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct MigrationGroup {
  pub(crate) name: String,
  pub(crate) version: i32,
}

impl MigrationGroup {
  /// Creates a new instance from all necessary parameters.
  #[inline]
  pub fn new<I>(version: i32, name: I) -> Self
  where
    I: Into<String>,
  {
    Self { name: name.into(), version }
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
    &self.name
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
  pub const fn version(&self) -> i32 {
    self.version
  }
}
