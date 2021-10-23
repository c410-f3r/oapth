use oapth::{DEFAULT_BATCH_SIZE, DEFAULT_ENV_VAR};
use std::string::String;

#[derive(Debug, argh::FromArgs)]
/// Oapth - CLI
pub(crate) struct Cli {
  /// configuration file path. If not specified, defaults to "oapth.cfg" in the current directory.
  #[argh(option, short = 'c')]
  pub(crate) cfg: Option<std::path::PathBuf>,

  #[argh(subcommand)]
  pub(crate) commands: Commands,

  /// number of files (migrations or seeds) that is going to be sent to the database in a
  /// single transaction.
  // Default value must match oapth::DEFAULT_BATCH_SIZE
  #[argh(default = "DEFAULT_BATCH_SIZE", option, short = 'f')]
  pub(crate) files_num: usize,

  /// seeds directory. This option is ignored by every command expect `migrate_and_seed` and
  /// `seed`.
  #[cfg(feature = "dev-tools")]
  #[argh(option, short = 's')]
  pub(crate) seeds: Option<std::path::PathBuf>,

  /// environment variable name that contains the database URL.
  #[argh(default = "DEFAULT_ENV_VAR.into()", option, short = 'v')]
  pub(crate) var: String,
}

#[derive(Debug, argh::FromArgs)]
#[argh(subcommand)]
pub(crate) enum Commands {
  #[cfg(feature = "dev-tools")]
  Clean(Clean),
  Migrate(Migrate),
  #[cfg(feature = "dev-tools")]
  MigrateAndSeed(MigrateAndSeed),
  Rollback(Rollback),
  #[cfg(feature = "dev-tools")]
  Seed(Seed),
  Validate(Validate),
}

/// Tries to clean all objects of a database, including separated namespaces/schemas.
#[derive(Debug, argh::FromArgs)]
#[argh(subcommand, name = "clean")]
#[cfg(feature = "dev-tools")]
pub(crate) struct Clean {}

/// Migrates everything that is greater than the last migration version within the database
#[derive(Debug, argh::FromArgs)]
#[argh(subcommand, name = "migrate")]
pub(crate) struct Migrate {}

/// Combines `migrate` and `seed` into a single command
#[derive(Debug, argh::FromArgs)]
#[argh(subcommand, name = "migrate-and-seed")]
pub(crate) struct MigrateAndSeed {}

/// Rollbacks the migrations to a given version
#[derive(Debug, argh::FromArgs)]
#[argh(subcommand, name = "rollback")]
pub(crate) struct Rollback {
  /// versions
  #[argh(option)]
  pub(crate) versions: Vec<i32>,
}

/// Seeds the database with arbitrary SQL
#[derive(Debug, argh::FromArgs)]
#[argh(subcommand, name = "seed")]
#[cfg(feature = "dev-tools")]
pub(crate) struct Seed {}

/// Verifies if all provided migrations exist in the database and have the same checksum
#[derive(Debug, argh::FromArgs)]
#[argh(subcommand, name = "validate")]
pub(crate) struct Validate {}
