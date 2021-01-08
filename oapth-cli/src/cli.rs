use alloc::string::String;
use oapth::{DEFAULT_BATCH_SIZE, DEFAULT_ENV_VAR};

#[derive(Debug, argh::FromArgs)]
/// Oapth - CLI
pub(crate) struct Cli {
  #[argh(subcommand)]
  pub(crate) commands: Commands,

  /// the number of files (migrations or seeds) that is going to be sent to the database in a
  /// single transaction.
  #[argh(default = "DEFAULT_BATCH_SIZE", option, short = 'f')]
  // Default value must match oapth::DEFAULT_BATCH_SIZE
  pub(crate) files_num: usize,

  /// the configuration file or seeds directory
  #[argh(option, short = 'p')]
  pub(crate) path: std::path::PathBuf,

  /// the environment variable name that contains the database URL.
  #[argh(default = "DEFAULT_ENV_VAR.into()", option, short = 'v')]
  pub(crate) var: String,
}

#[derive(Debug, argh::FromArgs)]
#[argh(subcommand)]
pub(crate) enum Commands {
  #[cfg(feature = "dev-tools")]
  Clean(Clean),
  Migrate(Migrate),
  Rollback(Rollback),
  #[cfg(feature = "dev-tools")]
  Seed(Seed),
  Validate(Validate),
}

/// Tries to clean all objects of a database, including separated namespaces/schemas.
#[argh(subcommand, name = "clean")]
#[cfg(feature = "dev-tools")]
#[derive(Debug, argh::FromArgs)]
pub(crate) struct Clean {}

/// Migrates everything that is greater than the last migration version within the database
#[argh(subcommand, name = "migrate")]
#[derive(Debug, argh::FromArgs)]
pub(crate) struct Migrate {}

/// Rollbacks the migrations to a given version
#[argh(subcommand, name = "rollback")]
#[derive(Debug, argh::FromArgs)]
pub(crate) struct Rollback {
  /// versions
  #[argh(option)]
  pub(crate) versions: Vec<i32>,
}

/// Seeds the database with arbitrary SQL
#[argh(subcommand, name = "seed")]
#[cfg(feature = "dev-tools")]
#[derive(Debug, argh::FromArgs)]
pub(crate) struct Seed {}

/// Verifies if all provided migrations exist in the database and have the same checksum
#[argh(subcommand, name = "validate")]
#[derive(Debug, argh::FromArgs)]
pub(crate) struct Validate {}
