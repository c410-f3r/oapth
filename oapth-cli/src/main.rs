//! oapth - CLI

mod cli;

use oapth::Config;
use std::{borrow::Cow, env::current_dir, path::Path};

const DEFAULT_CFG_FILE_NAME: &str = "oapth.cfg";

#[tokio::main]
async fn main() -> oapth::Result<()> {
  #[cfg(feature = "log")]
  env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
  #[cfg(feature = "dev-tools")]
  let _ = dotenv::dotenv().ok();

  let cli: cli::Cli = argh::from_env();
  let config = Config::with_url_from_var(&cli.var)?;

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

fn _cfg_file_path(cli: &cli::Cli) -> oapth::Result<Cow<'_, Path>> {
  Ok(if let Some(el) = cli.cfg.as_deref() {
    Cow::Borrowed(el)
  } else {
    let mut path_buf = current_dir()?;
    path_buf.push(DEFAULT_CFG_FILE_NAME);
    Cow::Owned(path_buf)
  })
}

#[inline]
async fn _handle_commands<B>(cli: &cli::Cli, back_end: B) -> oapth::Result<()>
where
  B: oapth::BackEnd,
{
  let mut commands = oapth::Commands::new(back_end, cli.files_num);
  match cli.commands {
    #[cfg(feature = "dev-tools")]
    cli::Commands::Clean(..) => {
      commands.clean().await?;
    }
    cli::Commands::Migrate(..) => {
      commands.migrate_from_cfg(&_cfg_file_path(cli)?).await?;
    }
    #[cfg(feature = "dev-tools")]
    cli::Commands::MigrateAndSeed(..) => {
      commands.migrate_from_cfg(&_cfg_file_path(cli)?).await?;
      commands.seed_from_dir(_require_seeds(cli)?).await?;
    }
    cli::Commands::Rollback(ref rollback) => {
      commands.rollback_from_cfg(&_cfg_file_path(cli)?, &rollback.versions).await?;
    }
    #[cfg(feature = "dev-tools")]
    cli::Commands::Seed(..) => {
      commands.seed_from_dir(_require_seeds(cli)?).await?;
    }
    cli::Commands::Validate(..) => {
      commands.validate_from_cfg(&_cfg_file_path(cli)?).await?;
    }
  }
  Ok(())
}

#[cfg(feature = "dev-tools")]
fn _require_seeds(cli: &cli::Cli) -> oapth::Result<&Path> {
  cli
    .seeds
    .as_deref()
    .ok_or(oapth::Error::Other("The requested command requires the `seeds` parameter"))
}
