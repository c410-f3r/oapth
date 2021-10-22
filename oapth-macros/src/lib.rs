//! Oapth macros

#![allow(clippy::missing_inline_in_public_items)]

#[cfg(feature = "embed-migrations")]
mod embed_migrations;

use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

const DEV_TOOLS: &str = "dev-tools";
const STD: &str = "std";
const WITH_DIESEL_MYSQL: &str = "with-diesel-mysql";
const WITH_DIESEL_PG: &str = "with-diesel-pg";
const WITH_DIESEL_SQLITE: &str = "with-diesel-sqlite";
const WITH_MYSQL_ASYNC: &str = "with-mysql_async";
const WITH_RUSQLITE: &str = "with-rusqlite";
const WITH_SQLX_MSSQL: &str = "with-sqlx-mssql";
const WITH_SQLX_MYSQL: &str = "with-sqlx-mysql";
const WITH_SQLX_PG: &str = "with-sqlx-pg";
const WITH_SQLX_SQLITE: &str = "with-sqlx-sqlite";
const WITH_TIBERIUS: &str = "with-tiberius";
const WITH_TOKIO_POSTGRES: &str = "with-tokio-postgres";

/// Embed migrations
#[cfg(feature = "embed-migrations")]
#[proc_macro]
pub fn embed_migrations(item: TokenStream) -> TokenStream {
  let err_tt = |s: &str| quote::quote!(compile_error!(#s)).into();

  macro_rules! manage_err {
    ($rslt:expr) => {
      match $rslt {
        Err(e) => {
          let s = e.to_string();
          return err_tt(&s);
        }
        Ok(e) => e,
      }
    };
  }

  let invalid_path_msg = "Please, provide a valid configuration path";
  let first_tt = manage_err!(item.into_iter().next().ok_or(invalid_path_msg));
  let literal = match first_tt {
    TokenTree::Literal(literal) => literal,
    _ => return err_tt(invalid_path_msg),
  };
  let literal_string = literal.to_string();
  let literal_str_opt = || {
    let len_minus_one = literal_string.len().checked_sub(1)?;
    literal_string.get(1..len_minus_one)
  };
  let literal_str = manage_err!(literal_str_opt().ok_or(invalid_path_msg));
  manage_err!(embed_migrations::embed_migrations(literal_str))
}

macro_rules! create_cfg {
  ($proc_macro_attribute:ident; $proc_macro:ident; $features:expr) => {
    /// Internal configuration. Doesn't have any meaningful public usage
    #[proc_macro]
    pub fn $proc_macro(item: TokenStream) -> TokenStream {
      create_cfg_generic($features, item)
    }

    /// Internal configuration. Doesn't have any meaningful public usage
    #[proc_macro_attribute]
    pub fn $proc_macro_attribute(_: TokenStream, item: TokenStream) -> TokenStream {
      create_cfg_generic($features, item)
    }
  };
}

macro_rules! create_features {
  ($($feature:expr),+) => {{
    let features = TokenStream::new();
    extend_features!(features; $($feature),+)
  }}
}

macro_rules! create_grouped_features {
  ($mac_name:ident; $group_name:literal; $($feature:expr),+) => {
    macro_rules! $mac_name {
      () => {
        create_grouped_features!($group_name; $($feature),+)
      }
    }
  };
  ($group_name:literal; $($feature:expr),+) => {{
    create_grouped_features!($group_name, create_features!($($feature),+))
  }};
  ($group_name:literal, $features:expr) => {{
    let mut tt = TokenStream::new();
    tt.extend(
      [
        TokenTree::Ident(Ident::new($group_name, Span::mixed_site())),
        TokenTree::Group(Group::new(Delimiter::Parenthesis, $features)),
        TokenTree::Punct(Punct::new(',', Spacing::Alone)),
      ].iter().cloned(),
    );
    tt
  }}
}

macro_rules! extend_features {
  ($tt:expr; $($feature:expr),+) => {{
    let mut maybe_expanded = $tt;
    maybe_expanded.extend([
      $(
        TokenTree::Ident(Ident::new("feature", Span::mixed_site())),
        TokenTree::Punct(Punct::new('=', Spacing::Alone)),
        TokenTree::Literal(Literal::string($feature)),
        TokenTree::Punct(Punct::new(',', Spacing::Alone)),
      )+
    ].iter().cloned());
    maybe_expanded
  }};
  ($tt:expr, $features:expr) => {{
    let mut maybe_expanded = $tt;
    maybe_expanded.extend($features);
    maybe_expanded
  }}
}

fn create_cfg_generic(features: TokenStream, item: TokenStream) -> TokenStream {
  let mut rslt = TokenStream::new();

  let mut cfg = TokenStream::new();
  cfg.extend(
    [
      TokenTree::Ident(Ident::new("cfg", Span::mixed_site())),
      TokenTree::Group(Group::new(Delimiter::Parenthesis, features)),
    ]
    .iter()
    .cloned(),
  );

  let mut cfg_outer = TokenStream::new();
  cfg_outer.extend(
    [
      TokenTree::Punct(Punct::new('#', Spacing::Alone)),
      TokenTree::Group(Group::new(Delimiter::Bracket, cfg)),
    ]
    .iter()
    .cloned(),
  );

  rslt.extend(cfg_outer);
  rslt.extend(item);

  rslt
}

// Any database

create_cfg!(
  _any_db;
  _any_db_;
  create_grouped_features!(
    "any";
    WITH_DIESEL_MYSQL,
    WITH_DIESEL_PG,
    WITH_DIESEL_SQLITE,
    WITH_MYSQL_ASYNC,
    WITH_RUSQLITE,
    WITH_SQLX_MSSQL,
    WITH_SQLX_MYSQL,
    WITH_SQLX_PG,
    WITH_SQLX_SQLITE,
    WITH_TIBERIUS,
    WITH_TOKIO_POSTGRES
  )
);

// Back end

create_cfg!(_diesel_mysql; _diesel_mysql_; create_features!(WITH_DIESEL_MYSQL));
create_cfg!(_diesel_pg; _diesel_pg_; create_features!(WITH_DIESEL_PG));
create_cfg!(_diesel_sqlite; _diesel_sqlite_; create_features!(WITH_DIESEL_SQLITE));
create_cfg!(_mysql_async; _mysql_async_; create_features!(WITH_MYSQL_ASYNC));
create_cfg!(_rusqlite; _rusqlite_; create_features!(WITH_RUSQLITE));
create_cfg!(_sqlx_mssql; _sqlx_mssql_; create_features!(WITH_SQLX_MSSQL));
create_cfg!(_sqlx_mysql; _sqlx_mysql_; create_features!(WITH_SQLX_MYSQL));
create_cfg!(_sqlx_pg; _sqlx_pg_; create_features!(WITH_SQLX_PG));
create_cfg!(_sqlx_sqlite; _sqlx_sqlite_; create_features!(WITH_SQLX_SQLITE));
create_cfg!(_tiberius; _tiberius_; create_features!(WITH_TIBERIUS));
create_cfg!(_tokio_postgres; _tokio_postgres_; create_features!(WITH_TOKIO_POSTGRES));

// Database

create_grouped_features!(
  mssql_any_features;
  "any";
  WITH_SQLX_MSSQL, WITH_TIBERIUS
);
create_cfg!(
  _mssql; _mssql_;
  mssql_any_features!()
);

create_grouped_features!(
  mysql_any_features;
  "any";
  WITH_DIESEL_MYSQL, WITH_MYSQL_ASYNC, WITH_SQLX_MYSQL
);
create_cfg!(
  _mysql; _mysql_;
  mysql_any_features!()
);

create_grouped_features!(
  pg_any_features;
  "any";
  WITH_DIESEL_PG, WITH_SQLX_PG, WITH_TOKIO_POSTGRES
);
create_cfg!(
  _pg; _pg_;
  pg_any_features!()
);

create_grouped_features!(
  sqlite_any_features;
  "any";
  WITH_DIESEL_SQLITE, WITH_RUSQLITE, WITH_SQLX_SQLITE
);
create_cfg!(
  _sqlite; _sqlite_;
  sqlite_any_features!()
);

// Diesel and SQLx

create_cfg!(
  _diesel; _diesel_;
  create_grouped_features!(
    "any";
    WITH_DIESEL_MYSQL, WITH_DIESEL_PG, WITH_DIESEL_SQLITE
  )
);
create_cfg!(
  _sqlx; _sqlx_;
  create_grouped_features!(
    "any";
    WITH_SQLX_MSSQL, WITH_SQLX_MYSQL, WITH_SQLX_PG, WITH_SQLX_SQLITE
  )
);

// Misc

create_cfg!(_dev_tools; _dev_tools_; create_features!(DEV_TOOLS));

create_cfg!(
  _diesel_minus_pg; _diesel_minus_pg_;
  create_grouped_features!(
    "any";
    WITH_DIESEL_MYSQL, WITH_DIESEL_SQLITE
  )
);

create_cfg!(_embed_migrations; _embed_migrations_; create_features!("embed-migrations"));

create_cfg!(
  _mysql_or_pg; _mysql_or_pg_;
  create_grouped_features!(
    "any",
    extend_features!(mysql_any_features!(), pg_any_features!())
  )
);

create_cfg!(
  _with_schema; _with_schema_;
  create_grouped_features!(
    "any";
    WITH_DIESEL_PG, WITH_SQLX_MSSQL, WITH_SQLX_PG, WITH_TIBERIUS, WITH_TOKIO_POSTGRES
  )
);

create_cfg!(
  _without_schema; _without_schema_;
  create_grouped_features!(
    "any";
    WITH_DIESEL_MYSQL, WITH_DIESEL_SQLITE, WITH_MYSQL_ASYNC, WITH_RUSQLITE, WITH_SQLX_MYSQL, WITH_SQLX_SQLITE
  )
);

create_cfg!(_std; _std_; create_features!(STD));
