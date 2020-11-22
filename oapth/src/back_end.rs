#[cfg(any(
  feature = "with-diesel-mysql",
  feature = "with-diesel-postgres",
  feature = "with-diesel-sqlite",
))]
pub(crate) mod diesel;
#[cfg(feature = "with-mysql_async")]
pub(crate) mod mysql_async;
#[cfg(feature = "with-rusqlite")]
pub(crate) mod rusqlite;
#[cfg(any(
  feature = "with-sqlx-mssql",
  feature = "with-sqlx-mysql",
  feature = "with-sqlx-postgres",
  feature = "with-sqlx-sqlite",
))]
pub(crate) mod sqlx;
#[cfg(feature = "tiberius")]
pub(crate) mod tiberius;
#[cfg(feature = "with-tokio-postgres")]
pub(crate) mod tokio_postgres;
pub(crate) mod unit;

/// Back end is the bridge between Rust and a database.
pub trait BackEnd: crate::_BackEnd {}
