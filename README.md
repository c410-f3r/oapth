# Oapth 

[![CI](https://github.com/c410-f3r/oapth/workflows/CI/badge.svg)](https://github.com/c410-f3r/oapth/actions?query=workflow%3ACI)
[![crates.io](https://img.shields.io/crates/v/oapth.svg)](https://crates.io/crates/oapth)
[![Documentation](https://docs.rs/oapth/badge.svg)](https://docs.rs/oapth)
[![License](https://img.shields.io/badge/license-APACHE2-blue.svg)](./LICENSE)
[![Rustc](https://img.shields.io/badge/rustc-1.48-lightgray")](https://blog.rust-lang.org/2020/03/12/Rust-1.48.html)

Flexible version control for databases through SQL migrations. Supports embedded and CLI workflows for MS-SQL, MariaDB, MySQL, PostgreSQL and SQLite.

This project is fully documented, applies fuzz tests in some targets and doesn't make use of `expect`, `panic`, `unsafe` or `unwrap`.

## Structure
 Just remember that the order of execution between migrations and migration groups is dictated by their numeric declaration order.
Each provided migration must contain, among other things, an unique version, a type, a name and a group summarized by the following illustration:

```txt
migrations
+-- 1__initial (Group)
    +-- 1__create_author.sql (Migration)
    +-- 2__create_post.sql (Migration)
+-- 2__fancy_stuff (Group)
    +-- 1__something_fancy.sql (Migration)
```

* `Group`: A set of migrations that must also contain an unique version and a name.
* `Migration`: A migration that is executed once and can't be modified.

The SQL file itself is composed by two parts, one for migrations (`-- oapth UP` section) and another for rollbacks (`-- oapth DOWN` section).

```sql
-- oapth UP

CREATE TABLE author (
  id INT NOT NULL PRIMARY KEY,
  added TIMESTAMP NOT NULL,
  birthdate DATE NOT NULL,
  email VARCHAR(100) NOT NULL,
  first_name VARCHAR(50) NOT NULL,
  last_name VARCHAR(50) NOT NULL
);

-- oapth DOWN

DROP TABLE author;
```

Execution order between migrations and migration groups is dictated by their numeric declaration order.

## No features by default

It is necessary to specify a desired feature to actually run the transactions, otherwise you will get a bunch of code that won't do much. Take a look at [Supported back ends](oapth#supported-back-ends).

```bash
cargo install oapth-cli
oapth migrate # Will do nothing
```

## Library example

The library uses `arrayvec`, `chrono` and `siphash` as mandatory internal crates which brings a total of 6 dependencies into your application. If this behavior is not acceptable, then you probably should discard the library and use the CLI binary instead as part of a custom deployment strategy.

```rust
// oapth = { features = ["with-sqlx-postgres", "with-sqlx-runtime-async-std"], version = "SOME_VERSION" }

use oapth::{Commands, Config, SqlxPostgres};
use std::path::Path;

#[async_std::main]
async fn main() -> oapth::Result<()> {
    let config = Config::with_url_from_default_var()?;
    let mut commands = Commands::new(SqlxPostgres::new(&config).await?);
    commands.migrate_from_dir(Path::new("migrations"), 128).await?;
    Ok(())
}
```

One thing worth noting is that these mandatory dependencies might already be part of your application as transients. In case of doubt, check your `Cargo.lock` file or type `cargo tree` for analysis.

## Supported back ends

Each back end has a feature that can be selected when using the library:

```bash
oapth = { features = ["with-tokio-postgres"], version = "SOME_VERSION" }
```

- mysql_async - `with-mysql_async`
- rusqlite - `with-rusqlite`
- Sqlx (MS-SQL) - `with-sqlx-mssql`
- Sqlx (MariaDB/MySql) - `with-sqlx-mysq`
- Sqlx (PostgreSQL) - `with-sqlx-postgres`
- Sqlx (SQLite) - `with-sqlx-sqlite`
- tokio-postgres - `with-tokio-postgres`

Or when installing the CLI binary:

```bash
cargo install oapth-cli --features "postgres"
```

- mssql
- mysql
- postgres
- sqlite

## Namespaces/Schemas

For supported databases, there is no direct user parameter that inserts migrations inside a single database schema but it is possible to specify the schema inside the SQL itself and arrange the migration groups structure in a way that most suits you.

```sql
-- oapth UP

CREATE SCHEMA cool_department_schema;

CREATE TABLE cool_department_schema.author (
  id INT NOT NULL PRIMARY KEY,
  full_name VARCHAR(50) NOT NULL
);

-- oapth DOWN

DROP TABLE cool_department_schema.author;
DROP SCHEMA cool_department_schema;
```
