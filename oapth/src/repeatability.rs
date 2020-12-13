use core::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
/// Migration repeatability
pub enum Repeatability {
  /// Always runs when executing a migration, regardless of the checksum
  Always,
  /// When executing a migration, runs if the checksum has been changed
  OnChecksumChange,
}

impl FromStr for Repeatability {
  type Err = crate::Error;

  #[inline]
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(match s {
      "always" => Self::Always,
      "on_checksum_change" => Self::OnChecksumChange,
      _ => return Err(crate::Error::IncompleteSqlFile),
    })
  }
}
