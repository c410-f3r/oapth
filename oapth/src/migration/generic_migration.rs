use crate::MigrationCommon;
use alloc::string::{String, ToString};
use core::hash::{Hash, Hasher};
use siphasher::sip::SipHasher13;

/// A migration that is intended to be inserted into a database

// Internally, `Migration` is anything that is NOT coming from the database
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct GenericMigration {
  pub(crate) common: MigrationCommon,
  pub(crate) sql_down: String,
  pub(crate) sql_up: String,
}

impl GenericMigration {
  #[inline]
  pub(crate) fn new(
    version: i32,
    name: String,
    sql_up: String,
    sql_down: String,
  ) -> GenericMigration {
    let mut hasher = SipHasher13::new();
    name.hash(&mut hasher);
    sql_up.hash(&mut hasher);
    sql_down.hash(&mut hasher);
    version.hash(&mut hasher);
    let checksum = hasher.finish().to_string();
    GenericMigration { common: MigrationCommon { checksum, name, version }, sql_down, sql_up }
  }
}
