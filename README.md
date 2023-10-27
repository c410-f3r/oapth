# Oapth

[![CI](https://github.com/c410-f3r/oapth/workflows/Tests/badge.svg)](https://github.com/c410-f3r/oapth/actions?query=workflow%3ATests)
[![crates.io](https://img.shields.io/crates/v/oapth.svg)](https://crates.io/crates/oapth)
[![Documentation](https://docs.rs/oapth/badge.svg)](https://docs.rs/oapth)
[![License](https://img.shields.io/badge/license-APACHE2-blue.svg)](./LICENSE)
[![Rustc](https://img.shields.io/badge/rustc-1.75-lightgray")](https://blog.rust-lang.org/2022/08/11/Rust-1.75.html)

Oapth is a suite of tools that interact with databases.

## Schema Manager

Using SQL migrations, supports embedded and CLI workflows for MS-SQL, MySQL, PostgreSQL and SQLite.

This project tries to support all database bridges of the Rust ecosystem, is fully documented, applies fuzz tests in some targets and doesn't make use of `expect`, `indexing`, `panic`, `unsafe` or `unwrap`.

### CLI

```bash
# Example

cargo install oapth-cli --features sm-dev,postgres --git https://github.com/c410-f3r/oapth
echo DATABASE_URL="postgres://USER:PW@localhost:5432/DB" > .env
RUST_LOG=debug oapth-cli migrate
```

The CLI application expects a configuration file that contains a set of paths where each path is a directory with multiple migrations.

```toml
# oapth.toml

migration_groups = [
  "migrations/1__initial",
  "migrations/2__fancy_stuff"
]
```

Each provided migration and group must contain an unique version and a name summarized by the following structure:

```txt
// Execution order of migrations is dictated by their numeric declaration order.

migrations
+-- 1__initial (Group)
    +-- 1__create_author.sql (Migration)
    +-- 2__create_post.sql (Migration)
+-- 2__fancy_stuff (Group)
    +-- 1__something_fancy.sql (Migration)
oapth.toml
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

One cool thing about the expected file configuration is that it can also be divided into smaller pieces, for example, the above migration could be transformed into `1__author_up.sql` and `1__author_down.sql`.

```sql
-- 1__author_up.sql

CREATE TABLE author (
  id INT NOT NULL PRIMARY KEY,
  added TIMESTAMP NOT NULL,
  birthdate DATE NOT NULL,
  email VARCHAR(100) NOT NULL,
  first_name VARCHAR(50) NOT NULL,
  last_name VARCHAR(50) NOT NULL
);
```

```sql
-- 1__author_down.sql

DROP TABLE author;
```

```txt
migrations
+-- 1__some_group (Group)
    +-- 1__author (Migration directory)
        +-- 1__author_down.sql (Down migration)
        +-- 1__author_up.sql (Up migration)
        +-- 1__author.toml (Optional configuration)
oapth.toml
```

### Library

The library gives freedom to arrange groups and uses some external crates, bringing ~10 additional dependencies into your application. If this overhead is not acceptable, then you probably should discard the library and use the CLI binary instead as part of a custom deployment strategy.

```rust
use oapth::{Commands, Config, SqlxPg};
use std::path::Path;

#[tokio::main]
async fn main() -> oapth::Result<()> {
  let config = Config::with_url_from_default_var()?;
  let mut commands = Commands::with_backend(SqlxPg::new(&config).await?);
  commands.migrate_from_dir(Path::new("my_custom_migration_group_path"), 128).await?;
  Ok(())
}
```

One thing worth noting is that these mandatory dependencies might already be part of your application as transients. In case of doubt, check your `Cargo.lock` file or type `cargo tree` for analysis.

### Embedded migrations

To make deployment easier, the final binary of your application can embed all necessary migrations by using the `embed_migrations!` macro that is available when selecting the `embed-migrations` feature.

```rust
use oapth::{Commands, Config, EmbeddedMigrationsTy, MysqlAsync, embed_migrations};

const MIGRATIONS: EmbeddedMigrationsTy = embed_migrations!("SOME_CONFIGURATION_FILE.toml");

#[tokio::main]
async fn main() -> oapth::Result<()> {
  let config = Config::with_url_from_default_var()?;
  let mut commands = Commands::with_backend(MysqlAsync::new(&config).await?);
  let groups = MIGRATIONS.iter().map(|e| (e.0, e.1.iter().cloned()));
  commands.migrate_from_groups(groups).await?;
  Ok(())
}
```

### Conditional migrations

If one particular migration needs to be executed in a specific set of databases, then it is possible to use the `-- oapth dbs` parameter in a file.

```sql
-- oapth dbs mssql,postgres

-- oapth UP

CREATE SCHEMA foo;

-- oapth DOWN

DROP SCHEMA foo;
```

### Repeatable migrations

Repeatability can be specified with `-- oapth repeatability SOME_VALUE` where `SOME_VALUE` can be either `always` (regardless of the checksum) or `on-checksum-change` (runs only when the checksums changes).

```sql
-- oapth dbs postgres
-- oapth repeatability always

-- oapth UP

CREATE OR REPLACE PROCEDURE something() LANGUAGE SQL AS $$ $$

-- oapth DOWN

DROP PROCEDURE something();
```

Keep in mind that repeatable migrations might break subsequent operations, therefore, you must known what you are doing. If desirable, they can be separated into dedicated groups.

```ini
migrations/1__initial_repeatable_migrations
migrations/2__normal_migrations
migrations/3__final_repeatable_migrations
```

### Namespaces/Schemas

For supported databases, there is no direct user parameter that inserts migrations inside a single database schema but it is possible to specify the schema inside the SQL file and arrange the migration groups structure in a way that most suits you.

```sql
-- oapth UP

CREATE TABLE cool_department_schema.author (
  id INT NOT NULL PRIMARY KEY,
  full_name VARCHAR(50) NOT NULL
);

-- oapth DOWN

DROP TABLE cool_department_schema.author;
```

## ORM

Currently only contains very basic support for CRUD operations.
