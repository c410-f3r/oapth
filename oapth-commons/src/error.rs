use core::fmt;

/// Wraps all possible errors related to `oapth` or third-party crates.
pub enum Error {
  /// Format error
  Fmt(fmt::Error),
  /// Incomplete SQL file
  IncompleteSqlFile,
  /// Invalid migration
  InvalidMigration,
  /// IO error
  #[cfg(feature = "std")]
  Io(std::io::Error),
  /// Maximum number of groups
  MaxNumGroups,
}

impl From<fmt::Error> for Error {
  #[inline]
  fn from(from: fmt::Error) -> Self {
    Self::Fmt(from)
  }
}

#[cfg(feature = "std")]
impl From<std::io::Error> for Error {
  #[inline]
  fn from(from: std::io::Error) -> Self {
    Self::Io(from)
  }
}

impl fmt::Debug for Error {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match *self {
      Self::Fmt(ref e) => write!(f, "Fmt: {}", e),
      Self::IncompleteSqlFile => write!(f, "A migration file must contain a '--oapth UP' section"),
      Self::InvalidMigration => write!(f, "Invalid migration"),
      #[cfg(feature = "std")]
      Self::Io(ref e) => write!(f, "IO: {}", e),
      Self::MaxNumGroups => write!(f, "There can't be more than 16 groups in a configuration"),
    }
  }
}

impl fmt::Display for Error {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Debug::fmt(self, f)
  }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
