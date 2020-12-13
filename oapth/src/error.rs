use core::fmt;

/// Wraps all possible errors related to `oapth` or third-party crates.
pub enum Error {
  #[cfg(any(
    feature = "with-diesel-mysql",
    feature = "with-diesel-pg",
    feature = "with-diesel-sqlite",
  ))]
  /// Diesel error
  Diesel(diesel::result::Error),
  #[cfg(any(
    feature = "with-diesel-mysql",
    feature = "with-diesel-pg",
    feature = "with-diesel-sqlite",
  ))]
  /// Diesel connection error
  DieselConnection(diesel::result::ConnectionError),
  /// Different rollback versions
  DifferentRollbackVersions,
  /// Format error
  Fmt(fmt::Error),
  /// Incomplete builder
  IncompleteBuilder,
  /// Incomplete SQL file
  IncompleteSqlFile,
  /// Incomplete migration builder
  IncompleteMigrationBuilder,
  /// Inexistent db migration
  InexistentDbMigration(i32),
  /// Invalid URL
  InvalidUrl,
  /// IO error
  #[cfg(feature = "std")]
  Io(std::io::Error),
  /// Missing environment variable
  MissingEnvVar,
  /// `mysql_async` error
  #[cfg(feature = "with-mysql_async")]
  MysqlAsync(mysql_async::Error),
  /// Other
  Other(&'static str),
  /// `rusqlite` error
  #[cfg(feature = "with-rusqlite")]
  Rusqlite(rusqlite::Error),
  /// `sqlx` error
  #[cfg(any(
    feature = "with-sqlx-mssql",
    feature = "with-sqlx-mysql",
    feature = "with-sqlx-pg",
    feature = "with-sqlx-sqlite",
  ))]
  Sqlx(sqlx_core::error::Error),
  /// `tiberius` error
  #[cfg(feature = "with-tiberius")]
  Tiberius(tiberius::error::Error),
  /// `tokio-postgres` error
  #[cfg(feature = "with-tokio-postgres")]
  TokioPostgres(tokio_postgres::Error),
  /// Validation - Divergent migrations
  ValidationDivergentMigrations(i32),
  /// Validation - Migrations number
  ValidationLessMigrationsNum(usize, usize),
}

#[oapth_macros::diesel_]
impl From<diesel::result::Error> for Error {
  #[inline]
  fn from(from: diesel::result::Error) -> Self {
    Self::Diesel(from)
  }
}

#[oapth_macros::diesel_]
impl From<diesel::result::ConnectionError> for Error {
  #[inline]
  fn from(from: diesel::result::ConnectionError) -> Self {
    Self::DieselConnection(from)
  }
}

impl From<fmt::Error> for Error {
  #[inline]
  fn from(from: fmt::Error) -> Self {
    Self::Fmt(from)
  }
}

#[cfg(feature = "with-mysql_async")]
impl From<mysql_async::Error> for Error {
  #[inline]
  fn from(from: mysql_async::Error) -> Self {
    Self::MysqlAsync(from)
  }
}

#[cfg(feature = "with-rusqlite")]
impl From<rusqlite::Error> for Error {
  #[inline]
  fn from(from: rusqlite::Error) -> Self {
    Self::Rusqlite(from)
  }
}

#[oapth_macros::sqlx_]
impl From<sqlx_core::error::Error> for Error {
  #[inline]
  fn from(from: sqlx_core::error::Error) -> Self {
    Self::Sqlx(from)
  }
}

#[cfg(feature = "std")]
impl From<std::io::Error> for Error {
  #[inline]
  fn from(from: std::io::Error) -> Self {
    Self::Io(from)
  }
}

#[oapth_macros::tiberius_]
impl From<tiberius::error::Error> for Error {
  #[inline]
  fn from(from: tiberius::error::Error) -> Self {
    Self::Tiberius(from)
  }
}

#[oapth_macros::tokio_postgres_]
impl From<tokio_postgres::Error> for Error {
  #[inline]
  fn from(from: tokio_postgres::Error) -> Self {
    Self::TokioPostgres(from)
  }
}

impl fmt::Debug for Error {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match *self {
      #[cfg(any(
        feature = "with-diesel-mysql",
        feature = "with-diesel-pg",
        feature = "with-diesel-sqlite",
      ))]
      Self::Diesel(ref e) => write!(f, "Diesel: {}", e),
      #[cfg(any(
        feature = "with-diesel-mysql",
        feature = "with-diesel-pg",
        feature = "with-diesel-sqlite",
      ))]
      Self::DieselConnection(ref e) => write!(f, "Diesel connection: {}", e),
      Self::DifferentRollbackVersions => write!(f, "The number of rollback versions must be equal the number of migration groups"),
      Self::Fmt(ref e) => write!(f, "Fmt: {}", e),
      Self::IncompleteBuilder => write!(f, "It is necessary to provide all parameters to the migration builder"),
      Self::IncompleteSqlFile => write!(
        f,
        "A migration file must contain a '--oapth UP' section"
      ),
      Self::IncompleteMigrationBuilder => write!(
        f,
        "It is necessary to fill all the `MigrationBuilder` parameters"
      ),
      Self::InexistentDbMigration(version) => write!(f, "Migration #{} doesn't exist in the database", version),
      Self::InvalidUrl => write!(f, "Url must start with the database type followed by a '://'"),
      #[cfg(feature = "std")]
      Self::Io(ref e) => write!(f, "IO: {}", e),
      Self::MissingEnvVar => {
        write!(f, "The environnement variable that contains the database url must be set")
      }
      #[cfg(feature = "with-mysql_async")]
      Self::MysqlAsync(ref e) => write!(f, "MySql: {}", e),
      Self::Other(s) => write!(f, "Other: {}", s),
      #[cfg(feature = "with-rusqlite")]
      Self::Rusqlite(ref e) => write!(f, "Rusqlite: {}", e),
      #[cfg(any(
        feature = "with-sqlx-mssql",
        feature = "with-sqlx-mysql",
        feature = "with-sqlx-pg",
        feature = "with-sqlx-sqlite",
      ))]
      Self::Sqlx(ref e) => write!(f, "SQLx: {}", e),
      #[cfg(feature = "with-tiberius")]
      Self::Tiberius(ref e) => write!(f, "tiberius: {}", e),
      #[cfg(feature = "with-tokio-postgres")]
      Self::TokioPostgres(ref e) => write!(f, "tokio postgres: {}", e),
      Self::ValidationDivergentMigrations(version) => {
        write!(
          f,
          "The provided migration #{version} has a checksum or name that is different than \
          the same #{version} migration in the database",
          version={version}
        )
      },
      Self::ValidationLessMigrationsNum(db_num, provided_num) => write!(
        f,
        "The number of provided migrations ({}) is less than the number of migrations in the database ({})",
        provided_num,
        db_num
      )
    }
  }
}

impl fmt::Display for Error {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Debug::fmt(self, f)
  }
}

#[oapth_macros::std_]
impl std::error::Error for Error {}
