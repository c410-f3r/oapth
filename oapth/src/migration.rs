pub mod db_migration;
pub mod migration_common;
pub mod migration_group;
pub mod migration_params;

use crate::{MigrationCommon, MigrationParams};
use alloc::string::{String, ToString};
use core::hash::{Hash, Hasher};
use siphasher::sip::SipHasher13;

/// A migration that is intended to be inserted into a database

// Internally, `Migration` is anything that is NOT coming from the database
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Migration {
  pub(crate) common: MigrationCommon,
  pub(crate) sql_down: String,
  pub(crate) sql_up: String,
}

impl MigrationParams for Migration {
  #[inline]
  fn common(&self) -> &MigrationCommon {
    &self.common
  }
}

impl Migration {
  /// Creates a new instance from all necessary input parameters.
  #[inline]
  pub fn new<IN, ISD, ISU>(version: i32, name: IN, sql_up: ISU, sql_down: ISD) -> Self
  where
    IN: Into<String>,
    ISD: Into<String>,
    ISU: Into<String>,
  {
    let _name = name.into();
    let _sql_up = sql_up.into();
    let _sql_down = sql_down.into();
    let mut hasher = SipHasher13::new();
    _name.hash(&mut hasher);
    _sql_up.hash(&mut hasher);
    _sql_down.hash(&mut hasher);
    version.hash(&mut hasher);
    let checksum = hasher.finish().to_string();
    Self {
      common: MigrationCommon { checksum, name: _name, version },
      sql_down: _sql_down,
      sql_up: _sql_up,
    }
  }

  /// Checksum
  ///
  /// # Example
  ///
  /// ```rust
  /// use oapth::doc_tests::migration;
  /// assert_eq!(migration().checksum(), "10126747658053090972")
  /// ```
  #[inline]
  pub fn checksum(&self) -> &str {
    &self.common.checksum
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
    &self.common.name
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
    &self.sql_down
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
    &self.sql_up
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
