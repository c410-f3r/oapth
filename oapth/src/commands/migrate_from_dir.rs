use crate::{map_paths_into_migrations, scan_canonical_migrations_dir, Backend, Commands};
use std::path::Path;

impl<B> Commands<B>
where
  B: Backend,
{
  /// Applies `migrate` to a set of groups according to the canonical directory structure.
  #[inline]
  pub async fn migrate_from_dir<'a>(
    &'a mut self,
    dir: &'a Path,
    files_num: usize,
  ) -> crate::Result<()> {
    self.backend.create_oapth_tables().await?;
    let mut buffer = Vec::with_capacity(16);
    for (mg, migrations_vec) in scan_canonical_migrations_dir(dir)? {
      let mut migrations = map_paths_into_migrations(migrations_vec.into_iter());
      loop_files!(buffer, migrations, files_num, self.do_migrate(&mg, buffer.iter()).await?);
    }
    Ok(())
  }
}

#[cfg(all(feature = "_integration_tests", test))]
mod tests {
  use crate::{Commands, Config};
  use std::path::Path;

  macro_rules! create_test {
    ($backend:ident) => {
      #[allow(non_snake_case)]
      #[tokio::test]
      async fn $backend() {
        let _ = env_logger::builder().is_test(true).try_init();
        let c = Config::with_url_from_default_var().unwrap();
        let backend = crate::$backend::new(&c).await.unwrap();
        let mut commands = Commands::new(backend);
        let path = Path::new("../oapth-test-utils/migrations");
        commands.migrate_from_dir(path, 128).await.unwrap();
      }
    };
  }

  #[cfg(feature = "with-mysql_async")]
  create_test!(MysqlAsync);
  #[cfg(feature = "with-rusqlite")]
  create_test!(Rusqlite);
  #[cfg(feature = "with-sqlx-mssql")]
  create_test!(SqlxMssql);
  #[cfg(feature = "with-sqlx-mysql")]
  create_test!(SqlxMysql);
  #[cfg(feature = "with-sqlx-postgres")]
  create_test!(SqlxPostgres);
  #[cfg(feature = "with-sqlx-sqlite")]
  create_test!(SqlxSqlite);
  #[cfg(feature = "with-tokio-postgres")]
  create_test!(TokioPostgres);
}
