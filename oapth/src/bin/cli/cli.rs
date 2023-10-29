use oapth::{sm::DEFAULT_BATCH_SIZE, DEFAULT_ENV_VAR};
use std::string::String;

#[derive(Debug, clap::Parser)]
/// Oapth - CLI
pub(crate) struct Cli {
  /// configuration file path. If not specified, defaults to "oapth.toml" in the current directory.
  #[arg(short = 'c')]
  pub(crate) _toml: Option<std::path::PathBuf>,

  #[command(subcommand)]
  pub(crate) _commands: Commands,

  /// number of files (migrations or seeds) that is going to be sent to the database in a
  /// single transaction.
  // Default value must match oapth::DEFAULT_BATCH_SIZE
  #[arg(default_value_t = DEFAULT_BATCH_SIZE, short = 'f')]
  pub(crate) _files_num: usize,

  /// seeds directory. If not specified, defaults to the optional directory specified in the
  /// configuration file.
  /// Returns an error if none of the options are available.
  #[cfg(feature = "sm-dev")]
  #[arg(short = 's')]
  pub(crate) _seeds: Option<std::path::PathBuf>,

  /// environment variable name that contains the database URL.
  #[arg(default_value_t = DEFAULT_ENV_VAR.into(), short = 'v')]
  pub(crate) _var: String,
}

#[allow(unused_tuple_struct_fields)]
#[derive(Debug, clap::Subcommand)]
pub(crate) enum Commands {
  #[cfg(feature = "sm-dev")]
  Clean {},
  Migrate {},
  #[cfg(feature = "sm-dev")]
  MigrateAndSeed {},
  Rollback {
    _versions: Vec<i32>,
  },
  #[cfg(feature = "sm-dev")]
  Seed {},
  Validate {},
}
