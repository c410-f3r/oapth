use alloc::string::String;

#[derive(Debug, clap::Clap)]
pub struct Cli {
  #[clap(subcommand)]
  pub(crate) commands: Commands,

  /// The migrations or seeds directory
  ///
  /// If seeding, defaults to "seeds", otherwise defaults to "migrations".
  #[clap(long, short = 'd')]
  pub(crate) dir: Option<std::path::PathBuf>,

  /// The environment variable name that contains the database URL.
  #[clap(default_value = "DATABASE_URL", long, short = 'e')]
  pub(crate) env_var: String,

  /// The number of files (migrations or seeds) that is going to be sent to the database in a
  /// single transaction.
  #[clap(default_value = "128", long, short = 'f')]
  pub(crate) files_num: usize,
}

#[derive(Debug, clap::Clap)]
pub enum Commands {
  /// Migrates everything that is greater than the last migration version within the database
  Migrate,
  #[cfg(feature = "dev-tools")]
  /// If existing, drops a given database and then re-creates it again.
  Reset {
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
