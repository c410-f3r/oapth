#[allow(unused_imports)]
use alloc::boxed::Box;
use core::fmt::{Debug, Display, Formatter};

/// Generic error
#[derive(Debug)]
pub enum Error {
  // External
  //
  /// See [arrayvec::ArrayVec]
  ArrayVec(arrayvec::CapacityError<()>),
  #[cfg(feature = "chrono")]
  /// See [chrono::ParseError]
  ChronoParseError(chrono::ParseError),
  /// See [core::fmt::Error]
  Fmt(core::fmt::Error),
  /// See [std::io::Error]
  #[cfg(feature = "std")]
  Io(std::io::Error),
  /// See [sqlx_core::error::Error].
  #[cfg(feature = "sqlx-core")]
  Sqlx(Box<sqlx_core::error::Error>),
  /// See [tiberius::error::Error].
  #[cfg(feature = "tiberius")]
  Tiberius(Box<tiberius::error::Error>),

  // Internal
  //
  /// The `seeds` parameter must be provided through the CLI or the configuration file.
  ChecksumMustBeANumber,
  /// Databases must be sorted and unique
  DatabasesMustBeSortedAndUnique,
  /// Different rollback versions
  DifferentRollbackVersions,
  /// Some internal operation found a hash collision of two table ids (likely) or a hash collision
  /// due to a number of nested associations larger than `MAX_NODES_NUM` (unlikely).
  HashCollision(&'static str, &'static str),
  /// Migration file has an empty attribute
  IncompleteSqlFile,
  /// Migration file has invalid syntax,
  InvalidMigration,
  /// An expected value could not be found
  InvalidDatabaseUrl(&'static str),
  /// Backend couldn't perform passed query string
  InvalidSqlQuery,
  /// Invalid URL
  InvalidUrl,
  /// Environment variable is not present
  MissingEnvVar,
  /// TOML parser only supports a subset of the official TOML specification
  TomlParserOnlySupportsStringsAndArraysOfStrings,
  /// TOML parser only supports a subset of the official TOML specification
  TomlValueIsTooLarge,
  /// Validation - Divergent migrations
  ValidationDivergentMigrations(i32),
  /// Validation - Migrations number
  ValidationLessMigrationsNum(usize, usize),
}

// Etc

impl From<Error> for () {
  #[inline]
  fn from(_: Error) -> Self {}
}

impl Display for Error {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    Debug::fmt(self, f)
  }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

// External

impl<T> From<arrayvec::CapacityError<T>> for Error {
  #[inline]
  #[track_caller]
  fn from(from: arrayvec::CapacityError<T>) -> Self {
    Self::ArrayVec(from.simplify())
  }
}

#[cfg(feature = "chrono")]
impl From<chrono::ParseError> for Error {
  #[inline]
  #[track_caller]
  fn from(from: chrono::ParseError) -> Self {
    Self::ChronoParseError(from)
  }
}

impl From<core::fmt::Error> for Error {
  #[inline]
  fn from(from: core::fmt::Error) -> Self {
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

#[cfg(feature = "sqlx-core")]
impl From<sqlx_core::error::Error> for Error {
  #[inline]
  fn from(from: sqlx_core::error::Error) -> Self {
    Self::Sqlx(from.into())
  }
}

#[cfg(feature = "tiberius")]
impl From<tiberius::error::Error> for Error {
  #[inline]
  fn from(from: tiberius::error::Error) -> Self {
    Self::Tiberius(from.into())
  }
}
