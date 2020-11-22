# Oapth 

[![CI](https://github.com/c410-f3r/oapth/workflows/Tests/badge.svg)](https://github.com/c410-f3r/oapth/actions?query=workflow%3ATests)
[![crates.io](https://img.shields.io/crates/v/oapth.svg)](https://crates.io/crates/oapth)
[![Documentation](https://docs.rs/oapth/badge.svg)](https://docs.rs/oapth)
[![License](https://img.shields.io/badge/license-APACHE2-blue.svg)](./LICENSE)
[![Rustc](https://img.shields.io/badge/rustc-stable-lightgray")](https://blog.rust-lang.org/2020/03/12/Rust-stable.html)

Flexible version control for databases through SQL migrations. Supports embedded and CLI workflows for MS-SQL, MariaDB, MySQL, PostgreSQL and SQLite.

This project tries to support all database bridges of the Rust ecosystem, is fully documented, applies fuzz tests in some targets and doesn't make use of `expect`, `panic`, `unsafe` or `unwrap`.

## No features by default

It is necessary to specify a desired feature to actually run the transactions, otherwise you will get a bunch of code that won't do much. Take a look at [Supported back ends](#supported-back-ends).

## CLI

The CLI application expects a configuration file that contains a set of paths where each path is a directory with multiple migrations.

```ini
// oapth.cfg

migrations/1__initial
migrations/2__create_post
```

Each provided migration and group must contain an unique version and a name summarized by the following structure:

```txt
migrations
+-- 1__initial (Group)
    +-- 1__create_author.sql (Migration)
    +-- 2__create_post.sql (Migration)
+-- 2__fancy_stuff (Group)
    +-- 1__something_fancy.sql (Migration)
oapth.cfg
```

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

## Library

The library gives freedom to arrange groups and uses `arrayvec`, `chrono` and `siphash` as mandatory internal crates which brings a total of 6 dependencies into your application. If this behavior is not acceptable, then you probably should discard the library and use the CLI binary instead as part of a custom deployment strategy.

```rust
// [dependencies]
// oapth = { features = ["with-sqlx-postgres"], version = "SOME_VERSION" }
// sqlx-core = { default-features = false, features = ["runtime-tokio-rustls"], version = "SOME_VERSION" }

use oapth::{Commands, Config, SqlxPostgres};
use std::path::Path;

#[async_std::main]
async fn main() -> oapth::Result<()> {
    let config = Config::with_url_from_default_var()?;
    let mut commands = Commands::new(SqlxPostgres::new(&config).await?);
    commands.migrate_from_dir(Path::new("my_custom_migration_group_path"), 128).await?;
    Ok(())
}
```

One thing worth noting is that these mandatory dependencies might already be part of your application as transients. In case of doubt, check your `Cargo.lock` file or type `cargo tree` for analysis.

## Supported back ends

Each back end has a feature that can be selected when using the library:

```bash
oapth = { features = ["with-tokio-postgres"], version = "SOME_VERSION" }
```

- Diesel (MariaDB/Mysql) - `with-diesel-mssql`
- Diesel (PostgreSQL) - `with-diesel-mysql`
- Diesel (SQlite) - `with-diesel-postgres`
- mysql_async - `with-mysql_async`
- rusqlite - `with-rusqlite`
- SQLx (MariaDB/MySql) - `with-sqlx-mysq`
- SQLx (MS-SQL) - `with-sqlx-mssql`
- SQLx (PostgreSQL) - `with-sqlx-postgres`
- SQLx (SQLite) - `with-sqlx-sqlite`
- tiberius - `with-tiberius`
- tokio-postgres - `with-tokio-postgres`

Or when installing the CLI binary:

```bash
cargo install oapth-cli --features "postgres"
```

- `mssql`
- `mysql`
- `postgres`
- `sqlite`

## Diesel support

Only migrations are supported and schema printing is still a work in progress. For any unsupported use-case, please use the official Diesel CLI binary.

## Namespaces/Schemas

For supported databases, there is no direct user parameter that inserts migrations inside a single database schema but it is possible to specify the schema inside the SQL file and arrange the migration groups structure in a way that most suits you.

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

##  Migration time zones

For PostgreSQL (except Diesel), migration timestamps are stored and retrieved with the timezone declared in the database. For everything else, timestamps are UTC.

| Back end                | Type             |
| ---------------------- | ---------------- |
| Diesel (MariaDB/Mysql) | UTC              |
| Diesel (PostgreSQL)    | UTC              |
| Diesel (SQlite)        | UTC              |
| mysql_async            | UTC              |
| rusqlite               | UTC              |
| SQLx (MariaDB/MySql)   | UTC              |
| SQLx (MS-SQL)          | UTC              |
| SQLx (PostgreSQL)      | Fixed time zones |
| SQLx (SQLite)          | UTC              |
| tiberius               | UTC              |
| tokio-postgres         | Fixed time zones |

## Development tools

- `clean`: Tries to clean all objects of a database, including separated namespaces/schemas.
- `seed`: Executes arbitrary code that is intended to populate data for tests.

These development tools are enabled with the `dev-tools` feature in both library and CLI.

## Project development

If you don't want to manually install all databases in your system, checkout `scripts/podman-start.sh` where each database image is pulled and executed automatically.