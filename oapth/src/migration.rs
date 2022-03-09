pub(crate) mod db_migration;
pub(crate) mod migration_common;
pub(crate) mod migration_group;

use crate::MigrationCommon;
use alloc::string::String;
use arrayvec::ArrayVec;
use core::hash::{Hash, Hasher};
use oapth_commons::{Database, Repeatability};
use siphasher::sip::SipHasher13;

/// Migration - Owned
pub type MigrationOwned = Migration<ArrayVec<Database, { Database::len() }>, String>;
/// Migration - Reference
pub type MigrationRef<'dbs, 'str> = Migration<&'dbs [Database], &'str str>;

/// A migration that is intended to be inserted into a database.
///
/// * Types
///
/// DBS: Databases
/// S: String
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Migration<DBS, S> {
  pub(crate) common: MigrationCommon<S>,
  pub(crate) dbs: DBS,
  pub(crate) sql_down: S,
  pub(crate) sql_up: S,
}

impl<DBS, S> Migration<DBS, S>
where
  DBS: AsRef<[Database]>,
  S: AsRef<str>,
{
  /// Creates a new instance from all necessary input parameters.
  #[inline]
  pub fn from_parts(
    dbs: DBS,
    repeatability: Option<Repeatability>,
    version: i32,
    name: S,
    sql_up: S,
    sql_down: S,
  ) -> crate::Result<Self> {
    if dbs.as_ref().windows(2).any(|s| s[0] >= s[1]) {
      return Err(crate::Error::DatabasesMustBeSortedAndUnique);
    }
    let mut hasher = SipHasher13::new();
    name.as_ref().hash(&mut hasher);
    sql_up.as_ref().hash(&mut hasher);
    sql_down.as_ref().hash(&mut hasher);
    version.hash(&mut hasher);
    let checksum = hasher.finish();
    Ok(Self {
      dbs,
      common: MigrationCommon { checksum, name, repeatability, version },
      sql_down,
      sql_up,
    })
  }

  /// Creates a new instance from all necessary input references.
  ///
  /// # Safety
  ///
  /// The caller of this function must include a valid checksum.
  #[allow(
    // Not used internally
    unsafe_code
  )]
  #[inline]
  pub const unsafe fn new_ref(
    checksum: u64,
    dbs: DBS,
    name: S,
    repeatability: Option<Repeatability>,
    sql_down: S,
    sql_up: S,
    version: i32,
  ) -> Self {
    Self {
      dbs,
      common: MigrationCommon { checksum, name, repeatability, version },
      sql_down,
      sql_up,
    }
  }

  /// Checksum
  ///
  /// # Example
  ///
  /// ```rust
  /// use oapth::doc_tests::migration;
  /// assert_eq!(migration().checksum(), 10126747658053090972)
  /// ```
  #[inline]
  pub fn checksum(&self) -> u64 {
    self.common.checksum
  }

  /// Databases
  ///
  /// An empty slice means all databases
  ///
  /// # Example
  ///
  /// ```rust
  /// use oapth::doc_tests::migration;
  /// assert_eq!(migration().dbs(), [])
  /// ```
  #[inline]
  pub fn dbs(&self) -> &[Database] {
    self.dbs.as_ref()
  }

  /// Name
  ///
  /// # Example
  ///
  /// ```rust
  /// use oapth::doc_tests::migration;
  /// assert_eq!(migration().name(), "create_author")
  /// ```
  #[inline]
  pub fn name(&self) -> &str {
    self.common.name.as_ref()
  }

  /// If this is a repeatable migration, returns its type.
  ///
  /// # Example
  ///
  /// ```rust
  /// use oapth::doc_tests::migration;
  /// assert_eq!(migration().repeatability(), None)
  /// ```
  #[inline]
  pub fn repeatability(&self) -> Option<Repeatability> {
    self.common.repeatability
  }

  /// Raw SQL for rollbacks
  ///
  /// # Example
  ///
  /// ```rust
  /// use oapth::doc_tests::migration;
  /// assert_eq!(migration().sql_down(), "DROP TABLE author");
  /// ```
  #[inline]
  pub fn sql_down(&self) -> &str {
    self.sql_down.as_ref()
  }

  /// Raw SQL for migrations
  ///
  /// # Example
  ///
  /// ```rust
  /// use oapth::doc_tests::migration;
  /// let mg = assert_eq!(
  ///   migration().sql_up(),
  ///   "CREATE TABLE author (id INT NOT NULL PRIMARY KEY, name VARCHAR(50) NOT NULL)"
  /// );
  #[inline]
  pub fn sql_up(&self) -> &str {
    self.sql_up.as_ref()
  }

  /// Migration version
  ///
  /// # Example
  ///
  /// ```rust
  /// use oapth::doc_tests::migration;
  /// assert_eq!(migration().version(), 1)
  /// ```
  #[inline]
  pub fn version(&self) -> i32 {
    self.common.version
  }
}
