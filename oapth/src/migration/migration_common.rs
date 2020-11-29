use alloc::string::String;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct MigrationCommon {
  pub(crate) checksum: String,
  pub(crate) name: String,
  pub(crate) version: i32,
}
