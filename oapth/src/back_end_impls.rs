oapth_macros::diesel! { pub(crate) mod diesel; }
oapth_macros::mysql_async! { pub(crate) mod mysql_async; }
oapth_macros::rusqlite! { pub(crate) mod rusqlite; }
oapth_macros::sqlx! { pub(crate) mod sqlx; }
oapth_macros::tiberius! { pub(crate) mod tiberius; }
oapth_macros::tokio_postgres! { pub(crate) mod tokio_postgres; }
pub(crate) mod unit;
