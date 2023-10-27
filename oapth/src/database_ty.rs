create_enum! {
  /// Database
  #[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
  pub enum DatabaseTy {
    /// MS-SQL
    Mssql, "mssql";
    /// MySql
    Mysql, "mysql";
    /// PostgreSQL
    Postgres, "postgres";
    /// SQLite
    Sqlite, "sqlite";
    /// Unit (Dummy used for testing)
    Unit, "unit";
  }
}
