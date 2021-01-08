//! oapth - CLI

extern crate alloc;

mod cli;

use oapth::Config;

#[tokio::main]
async fn main() -> oapth::Result<()> {
  let cli: cli::Cli = argh::from_env();
  let config = Config::with_url_from_var(&cli.var)?;
  match config.database()? {
    "mariadb" | "mysql" => {
      #[cfg(feature = "mysql")]
      {
        let back_end = oapth::MysqlAsync::new(&config).await?;
        _handle_commands(&cli, back_end).await?
      }
      #[cfg(not(feature = "mysql"))]
      eprintln!("No feature enabled for MySQL-like databases");
    }
    "mssql" | "sqlserver" => {
      #[cfg(feature = "mssql")]
      {
        let back_end = oapth::SqlxMssql::new(&config).await?;
        _handle_commands(&cli, back_end).await?
      }
      #[cfg(not(feature = "mssql"))]
      eprintln!("No feature enabled for MS-SQL");
    }
    "postgres" | "postgresql" => {
      #[cfg(feature = "pg")]
      {
        let back_end = oapth::TokioPostgres::new(&config).await?;
        _handle_commands(&cli, back_end).await?
      }
      #[cfg(not(feature = "pg"))]
      eprintln!("No feature enabled for PostgreSQL");
    }
    "sqlite" => {
      #[cfg(feature = "sqlite")]
      {
        let back_end = oapth::Rusqlite::new(&config).await?;
        _handle_commands(&cli, back_end).await?
      }
      #[cfg(not(feature = "sqlite"))]
      eprintln!("No feature enabled for SQLite");
    }
    _ => return Err(oapth::Error::InvalidUrl),
  }
  Ok(())
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
      commands.migrate_from_cfg(&cli.path).await?;
    }
    cli::Commands::Rollback(ref rollback) => {
      commands.rollback_from_cfg(&cli.path, &rollback.versions).await?;
    }
    #[cfg(feature = "dev-tools")]
    cli::Commands::Seed(..) => {
      commands.seed_from_dir(&cli.path).await?;
    }
    cli::Commands::Validate(..) => {
      commands.validate_from_cfg(&cli.path).await?;
    }
  }
  Ok(())
}
