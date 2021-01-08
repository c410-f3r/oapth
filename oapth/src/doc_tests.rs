//! Instances mostly used for documentation tests

use crate::{MigrationGroupRef, MigrationRef};

/// ```rust
/// let _ = oapth::MigrationRef::from_parts(
///   &[],
///   None,
///   1,
///   "create_author",
///   "CREATE TABLE author (id INT NOT NULL PRIMARY KEY, name VARCHAR(50) NOT NULL)",
///   "DROP TABLE author",
/// );
/// ```
#[inline]
pub fn migration() -> MigrationRef<'static, 'static> {
  MigrationRef::from_parts(
    &[],
    None,
    1,
    "create_author",
    "CREATE TABLE author (id INT NOT NULL PRIMARY KEY, name VARCHAR(50) NOT NULL)",
    "DROP TABLE author",
  )
  .unwrap()
}

/// ```rust
/// let _ = oapth::MigrationGroupRef::new("initial", 1);
/// ```
#[inline]
pub fn migration_group() -> MigrationGroupRef<'static> {
  MigrationGroupRef::new("initial", 1)
}
