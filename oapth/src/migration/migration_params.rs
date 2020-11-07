use crate::MigrationCommon;

pub trait MigrationParams {
  fn common(&self) -> &MigrationCommon;
}
