create_enum! {
  /// Database
  #[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
  pub enum Database {
    /// MS-SQL
    Mssql, "mssql";
    /// MySql
    Mysql, "mysql";
    /// PostgreSQL
    Pg, "pg";
    /// SQLite
    Sqlite, "sqlite";
    /// Unit (Dummy used for testing)
    Unit, "unit";
  }
}
