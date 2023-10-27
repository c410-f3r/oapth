use oapth::sm::utils::{group_and_migrations_from_path, parse_root_toml};
use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use std::path::Path;

pub(crate) fn embed_migrations(cfg_path_str: &str) -> oapth::Result<TokenStream> {
  let (mut migration_groups, _) = parse_root_toml(Path::new(cfg_path_str))?;
  let mut groups_and_migrations = Vec::new();

  migration_groups.sort();
  let mut inner = Vec::new();

  for mg_path in migration_groups {
    let (mg, ms) = group_and_migrations_from_path(&mg_path, Ord::cmp)?;
    let mg_name = mg.name();
    let mg_version = mg.version();
    let filtered = ms
      .filter_map(|e| {
        let migration = e.ok()?;
        let checksum = migration.checksum();
        let dbs = migration.dbs();
        let name = migration.name();
        let repeatability = migration.repeatability();
        let sql_down = migration.sql_down();
        let sql_up = migration.sql_up();
        let version = migration.version();
        let opt_hack = QuoteOption(repeatability);
        Some(quote! { ( #checksum, &[#(#dbs,)*], #name, #opt_hack, #sql_down, #sql_up, #version ) })
      })
      .collect::<Vec<_>>();

    let mg_ident = format_ident!("{}", mg_name);
    let ms_ident = format_ident!("{}_MIGRATIONS", mg_name);

    let quote = quote! {
      const #mg_ident: &oapth::MigrationGroupRef<'static> = &oapth::MigrationGroupRef::new(
        #mg_name, #mg_version
      );

      const #ms_ident: &[oapth::MigrationRef<'static, 'static>] = &[#(unsafe {
        let tuple = #filtered;
        oapth::Migration::new(
          tuple.0,
          tuple.1,
          tuple.2,
          tuple.3,
          tuple.4,
          tuple.5,
          tuple.6,
        )
      },)*];
    };

    groups_and_migrations.push(quote! { (#mg_ident, #ms_ident) });
    inner.push(quote);
  }

  Ok(Into::<TokenStream>::into(quote! {
    {
      #(#inner)*

      &[#(#groups_and_migrations,)*][..]
    }
  }))
}

struct QuoteOption<T>(Option<T>);

impl<T> ToTokens for QuoteOption<T>
where
  T: ToTokens,
{
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    tokens.append_all(if let Some(t) = &self.0 {
      quote! { core::option::Option::Some(#t) }
    } else {
      quote! { core::option::Option::None }
    });
  }
}
