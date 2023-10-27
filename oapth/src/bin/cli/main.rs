//! oapth - CLI

mod cli;

use oapth::{
  sm::{Config, DbMigration},
  Identifier,
};
use std::{borrow::Cow, env::current_dir, path::Path};

const _DEFAULT_CFG_FILE_NAME: &str = "oapth.toml";

#[tokio::main]
async fn main() -> oapth::Result<()> {
  #[cfg(feature = "env_logger")]
  env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
  #[cfg(feature = "sm-dev")]
  let _ = dotenv::dotenv().ok();

  let cli: cli::Cli = argh::from_env();
  let config = Config::with_url_from_var(&cli._var)?;

  match config.database()? {
    "mysql" => {
      #[cfg(feature = "mysql")]
      _handle_commands(&cli, oapth::database::SqlxMysql::new(&config).await?).await?;
      #[cfg(not(feature = "mysql"))]
      eprintln!("No feature enabled for MySQL-like databases");
    }
    "mssql" | "sqlserver" => {
      #[cfg(feature = "mssql")]
      _handle_commands(&cli, oapth::database::Tiberius::new(&config).await?).await?;
      #[cfg(not(feature = "mssql"))]
      eprintln!("No feature enabled for MS-SQL");
    }
    "postgres" | "postgresql" => {
      #[cfg(feature = "postgres")]
      _handle_commands(&cli, oapth::database::SqlxPostgres::new(&config).await?).await?;
      #[cfg(not(feature = "postgres"))]
      eprintln!("No feature enabled for PostgreSQL");
    }
    "sqlite" => {
      #[cfg(feature = "sqlite")]
      _handle_commands(&cli, oapth::database::SqlxSqlite::new(&config).await?).await?;
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
async fn _handle_commands<D>(
  (buffer_cmd, buffer_db_migrations, buffer_idents): (
    &mut String,
    &mut Vec<DbMigration>,
    &mut Vec<Identifier>,
  ),
  cli: &cli::Cli,
  database: D,
) -> oapth::Result<()>
where
  D: oapth::sm::SchemaManagement,
{
  let mut commands = oapth::sm::Commands::new(cli._files_num, database);
  match cli._commands {
    cli::Commands::Clean(..) => {
      commands.clear((buffer_cmd, buffer_idents)).await?;
    }
    cli::Commands::Migrate(..) => {
      commands
        .migrate_from_toml_path((buffer_cmd, buffer_db_migrations), &_toml_file_path(cli)?)
        .await?;
    }
    #[cfg(feature = "sm-dev")]
    cli::Commands::MigrateAndSeed(..) => {
      let (migration_groups, seeds) = oapth::sm::utils::parse_root_toml(&_toml_file_path(cli)?)?;
      commands
        .migrate_from_groups_paths((buffer_cmd, buffer_db_migrations), &migration_groups)
        .await?;
      commands.seed_from_dir(buffer_cmd, _seeds_file_path(cli, seeds.as_deref())?).await?;
    }
    cli::Commands::Rollback(ref rollback) => {
      commands
        .rollback_from_toml(
          (buffer_cmd, buffer_db_migrations),
          &_toml_file_path(cli)?,
          &rollback._versions,
        )
        .await?;
    }
    #[cfg(feature = "sm-dev")]
    cli::Commands::Seed(..) => {
      let (_, seeds) = oapth::sm::utils::parse_root_toml(&_toml_file_path(cli)?)?;
      commands.seed_from_dir(buffer_cmd, _seeds_file_path(cli, seeds.as_deref())?).await?;
    }
    cli::Commands::Validate(..) => {
      commands
        .validate_from_toml((buffer_cmd, buffer_db_migrations), &_toml_file_path(cli)?)
        .await?;
    }
  }
  Ok(())
}

#[cfg(feature = "sm-dev")]
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
  panic!("The `seeds` parameter must be provided through the CLI or the configuration file");
}
