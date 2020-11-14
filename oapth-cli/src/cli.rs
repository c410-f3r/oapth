use alloc::string::String;

#[derive(Debug, clap::Clap)]
pub struct Cli {
  #[clap(subcommand)]
  pub(crate) commands: Commands,

  /// The environment variable name that contains the database URL.
  #[clap(default_value = "DATABASE_URL", long, short = 'e')]
  pub(crate) env_var: String,

  /// The number of files (migrations or seeds) that is going to be sent to the database in a
  /// single transaction.
  #[clap(default_value = "128", long, short = 'f')]
  pub(crate) files_num: usize,

  /// The configuration file or seeds directory
  ///
  /// If seeding, defaults to "seeds", otherwise defaults to "oapth.cfg".
  #[clap(long, short = 'p')]
  pub(crate) path: Option<std::path::PathBuf>,
}

#[derive(Debug, clap::Clap)]
pub enum Commands {
  /// Migrates everything that is greater than the last migration version within the database
  Migrate,
  #[cfg(feature = "dev-tools")]
  /// Attempts to drop a given database and then recreates it again.
  Recreate {
    /// Database name
    name: String,
  },
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
