//! oapth - CLI

extern crate alloc;

mod cli;
mod error;

use clap::Clap;
use error::*;
use oapth::Config;
use std::path::{Path, PathBuf};

type Result<T> = core::result::Result<T, Error>;

#[tokio::main]
async fn main() -> Result<()> {
  let cli = cli::Cli::parse();
  let config = Config::with_url_from_var(&cli.env_var)?;
  let db = config.url().split(':').next().ok_or(oapth::Error::InvalidUrl)?;
  match db {
    "mariadb" | "mysql" => {
      #[cfg(feature = "mysql")]
      {
        let backend = oapth::MysqlAsync::new(&config).await?;
        _handle_commands(&cli, backend).await?
      }
      #[cfg(not(feature = "mysql"))]
      eprintln!("No feature enabled for MySQL-like databases");
    }
    "mssql" | "sqlserver" => {
      #[cfg(feature = "mssql")]
      {
        let backend = oapth::MysqlAsync::new(&config).await?;
        _handle_commands(&cli, backend).await?
      }
      #[cfg(not(feature = "mssql"))]
      eprintln!("No feature enabled for SQL Server");
    }
    "postgres" | "postgresql" => {
      #[cfg(feature = "postgres")]
      {
        let backend = oapth::TokioPostgres::new(&config).await?;
        _handle_commands(&cli, backend).await?
      }
      #[cfg(not(feature = "postgres"))]
      eprintln!("No feature enabled for PostgreSQL");
    }
    "sqlite" => {
      #[cfg(feature = "sqlite")]
      {
        let backend = oapth::Rusqlite::new(&config).await?;
        _handle_commands(&cli, backend).await?
      }
      #[cfg(not(feature = "postgres"))]
      eprintln!("No feature enabled for SQLite");
    }
    _ => return Err(oapth::Error::InvalidUrl.into()),
  }
  Ok(())
}

#[inline]
async fn _handle_commands<'a, B>(cli: &cli::Cli, backend: B) -> Result<()>
where
  B: oapth::Backend + 'a,
{
  let mut commands = oapth::Commands::new(backend);
  match cli.commands {
    cli::Commands::Migrate => {
      let dir = _migrations_dir(cli.dir.as_ref());
      commands.migrate_from_dir(dir, cli.files_num).await?;
    }
    #[cfg(feature = "dev-tools")]
    cli::Commands::Reset { ref name } => {
      commands.reset(name).await?;
    }
    cli::Commands::Rollback { ref versions } => {
      let dir = _migrations_dir(cli.dir.as_ref());
      commands.rollback_from_dir(dir, versions.iter().copied(), cli.files_num).await?;
    }
    #[cfg(feature = "dev-tools")]
    cli::Commands::Seed => {
      let dir = _seeds_dir(cli.dir.as_ref());
      commands.seed_from_dir(dir).await?;
    }
    cli::Commands::Validate => {
      let dir = _migrations_dir(cli.dir.as_ref());
      commands.validate_from_dir(dir, cli.files_num).await?;
    }
  }
  Ok(())
}

#[inline]
fn _migrations_dir(path: Option<&PathBuf>) -> &Path {
  if let Some(rslt) = path {
    rslt.as_path()
  } else {
    Path::new("migrations")
  }
}

#[inline]
fn _seeds_dir(path: Option<&PathBuf>) -> &Path {
  if let Some(rslt) = path {
    rslt.as_path()
  } else {
    Path::new("seeds")
  }
}
