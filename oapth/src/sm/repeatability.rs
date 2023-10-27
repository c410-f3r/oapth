create_enum! {
  /// Migration repeatability
  #[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
  pub enum Repeatability {
    /// Always runs when executing a migration, regardless of the checksum
    Always, "always";
    /// When executing a migration, runs if the checksum has been changed
    OnChecksumChange, "on-checksum-change";
  }
}
