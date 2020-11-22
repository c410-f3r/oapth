use alloc::string::String;

#[derive(Debug, clap::Clap)]
pub struct Cli {
  #[clap(subcommand)]
  pub(crate) commands: Commands,

  /// The number of files (migrations or seeds) that is going to be sent to the database in a
  /// single transaction.
  #[clap(default_value = "128", long, short = 'f')]
  pub(crate) files_num: usize,

  /// The configuration file or seeds directory
  #[clap(long, required = true, short = 'p')]
  pub(crate) path: std::path::PathBuf,

  /// The environment variable name that contains the database URL.
  #[clap(default_value = "DATABASE_URL", long, short = 'v')]
  pub(crate) var: String,
}

#[derive(Debug, clap::Clap)]
pub enum Commands {
  /// Tries to clean all objects of a database, including separated namespaces/schemas.
  #[cfg(feature = "dev-tools")]
  Clean,
  /// Migrates everything that is greater than the last migration version within the database
  Migrate,
  /// Rollbacks the migrations to a given version
  Rollback {
    /// Versions
    versions: Vec<i32>,
  },
  /// Seeds the database with arbitrary SQL
  #[cfg(feature = "dev-tools")]
  Seed,
  /// Verifies if all provided migrations exist in the database and have the same checksum
  Validate,
}
