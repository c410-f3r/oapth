//! oapth - CLI

mod cli;

use oapth::Config;
use std::{borrow::Cow, env::current_dir, path::Path};

const _DEFAULT_CFG_FILE_NAME: &str = "oapth.toml";

#[tokio::main]
async fn main() -> oapth::Result<()> {
  #[cfg(feature = "log")]
  env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
  #[cfg(feature = "dev-tools")]
  let _ = dotenv::dotenv().ok();

  let cli: cli::Cli = argh::from_env();
  let config = Config::with_url_from_var(&cli._var)?;

  match config.database()? {
    "mariadb" | "mysql" => {
      #[cfg(feature = "mysql")]
      _handle_commands(&cli, oapth::MysqlAsync::new(&config).await?).await?;
      #[cfg(not(feature = "mysql"))]
      eprintln!("No feature enabled for MySQL-like databases");
    }
    "mssql" | "sqlserver" => {
      #[cfg(feature = "mssql")]
      _handle_commands(&cli, oapth::SqlxMssql::new(&config).await?).await?;
      #[cfg(not(feature = "mssql"))]
      eprintln!("No feature enabled for MS-SQL");
    }
    "postgres" | "postgresql" => {
      #[cfg(feature = "pg")]
      _handle_commands(&cli, oapth::TokioPostgres::new(&config).await?).await?;
      #[cfg(not(feature = "pg"))]
      eprintln!("No feature enabled for PostgreSQL");
    }
    "sqlite" => {
      #[cfg(feature = "sqlite")]
      _handle_commands(&cli, oapth::Rusqlite::new(&config).await?).await?;
      #[cfg(not(feature = "sqlite"))]
      eprintln!("No feature enabled for SQLite");
    }
    _ => return Err(oapth::Error::InvalidUrl),
  }
  Ok(())
}

fn _toml_file_path(cli: &cli::Cli) -> oapth::Result<Cow<'_, Path>> {
  Ok(if let Some(el) = cli._toml.as_deref() {
    Cow::Borrowed(el)
  } else {
    let mut path_buf = current_dir()?;
    path_buf.push(_DEFAULT_CFG_FILE_NAME);
    Cow::Owned(path_buf)
  })
}

#[inline]
async fn _handle_commands<B>(cli: &cli::Cli, backend: B) -> oapth::Result<()>
where
  B: oapth::Backend,
{
  let mut buffer = String::new();
  let mut commands = oapth::Commands::new(backend, cli._files_num);
  match cli._commands {
    #[cfg(feature = "dev-tools")]
    cli::Commands::Clean(..) => {
      commands.clean(&mut buffer).await?;
    }
    cli::Commands::Migrate(..) => {
      commands.migrate_from_toml_path(&mut buffer, &_toml_file_path(cli)?).await?;
    }
    #[cfg(feature = "dev-tools")]
    cli::Commands::MigrateAndSeed(..) => {
      let (migration_groups, seeds) = oapth_commons::parse_root_toml(&_toml_file_path(cli)?)?;
      commands.migrate_from_groups_paths(&mut buffer, migration_groups).await?;
      commands.seed_from_dir(_seeds_file_path(cli, seeds.as_deref())?).await?;
    }
    cli::Commands::Rollback(ref rollback) => {
      commands.rollback_from_toml(&mut buffer, &_toml_file_path(cli)?, &rollback._versions).await?;
    }
    #[cfg(feature = "dev-tools")]
    cli::Commands::Seed(..) => {
      let (_, seeds) = oapth_commons::parse_root_toml(&_toml_file_path(cli)?)?;
      commands.seed_from_dir(_seeds_file_path(cli, seeds.as_deref())?).await?;
    }
    cli::Commands::Validate(..) => {
      commands.validate_from_toml(&_toml_file_path(cli)?).await?;
    }
  }
  Ok(())
}

#[cfg(feature = "dev-tools")]
fn _seeds_file_path<'a, 'b, 'c>(
  cli: &'a cli::Cli,
  seeds_toml: Option<&'b Path>,
) -> oapth::Result<&'c Path>
where
  'a: 'c,
  'b: 'c,
{
  if let Some(el) = cli._seeds.as_deref() {
    return Ok(el);
  }
  if let Some(el) = seeds_toml {
    return Ok(el);
  }
  Err(oapth::Error::Other(
    "The `seeds` parameter must be provided through the CLI or the configuration file",
  ))
}
