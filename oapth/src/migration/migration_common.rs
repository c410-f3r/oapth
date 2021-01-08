use alloc::string::String;
use oapth_commons::Repeatability;

pub(crate) type MigrationCommonOwned = MigrationCommon<String>;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct MigrationCommon<S> {
  pub(crate) checksum: u64,
  pub(crate) name: S,
  pub(crate) repeatability: Option<Repeatability>,
  pub(crate) version: i32,
}
