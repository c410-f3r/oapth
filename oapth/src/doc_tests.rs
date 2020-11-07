//! Instances for documentation tests

use crate::{Migration, MigrationGroup};

/// ```rust
/// let _ = oapth::Migration::new(
///    1,
///    "create_author",
///    "CREATE TABLE author (id INT NOT NULL PRIMARY KEY, name VARCHAR(50) NOT NULL)",
///    "DROP TABLE author",
/// );
/// ```
#[inline]
pub fn migration() -> Migration {
  Migration::new(
    1,
    "create_author",
    "CREATE TABLE author (id INT NOT NULL PRIMARY KEY, name VARCHAR(50) NOT NULL)",
    "DROP TABLE author",
  )
}

/// ```rust
/// let _ = oapth::MigrationGroup::new(1, "initial");
/// ```
#[inline]
pub fn migration_group() -> MigrationGroup {
  MigrationGroup::new(1, "initial")
}
