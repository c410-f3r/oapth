use alloc::string::String;
use core::cmp::Ordering;

#[derive(Debug)]
pub struct MigrationCommon {
  pub(crate) checksum: String,
  pub(crate) name: String,
  pub(crate) version: i32,
}

impl Eq for MigrationCommon {}

impl PartialEq for MigrationCommon {
  #[inline]
  fn eq(&self, other: &MigrationCommon) -> bool {
    self.checksum == other.checksum && self.name == other.name && self.version == other.version
  }
}

impl PartialOrd for MigrationCommon {
  #[inline]
  fn partial_cmp(&self, other: &MigrationCommon) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for MigrationCommon {
  #[inline]
  fn cmp(&self, other: &MigrationCommon) -> Ordering {
    match self.checksum.cmp(&other.checksum) {
      Ordering::Equal => match self.name.cmp(&other.name) {
        Ordering::Equal => match self.version.cmp(&other.version) {
          Ordering::Equal => Ordering::Equal,
          cmp @ Ordering::Less | cmp @ Ordering::Greater => cmp,
        },
        cmp @ Ordering::Less | cmp @ Ordering::Greater => cmp,
      },
      cmp @ Ordering::Less | cmp @ Ordering::Greater => cmp,
    }
  }
}
