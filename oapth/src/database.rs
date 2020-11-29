use core::str::FromStr;

/// Database
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Database {
  /// MS-SQL
  Mssql,
  /// MySql
  Mysql,
  /// PostgreSQL
  Pg,
  /// SQLite
  Sqlite,
}

impl FromStr for Database {
  type Err = ();

  #[inline]
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(match s {
      "mssql" => Self::Mssql,
      "mysql" => Self::Mysql,
      "pg" => Self::Pg,
      "sqlite" => Self::Sqlite,
      _ => return Err(()),
    })
  }
}
