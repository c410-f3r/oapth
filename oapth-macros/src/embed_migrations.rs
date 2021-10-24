use oapth_commons::{group_and_migrations_from_path, parse_root_toml};
use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use std::path::Path;

pub(crate) fn embed_migrations(cfg_path_str: &str) -> oapth_commons::Result<TokenStream> {
  let (mut migration_groups, _) = parse_root_toml(Path::new(cfg_path_str))?;
  let mut groups_and_migrations = Vec::new();

  migration_groups.sort();
  let mut ts = TokenStream::new();

  for mg in migration_groups {
    let ((mg_name, mg_version), ms) = group_and_migrations_from_path(&mg, |a, b| a.cmp(b))?;
    let filtered = ms
      .filter_map(|e| {
        let (checksum, dbs, name, repeatability, sql_down, sql_up, version) = e.ok()?;
        let opt_hack = QuoteOption(repeatability);
        Some(quote! { ( #checksum, &[#(#dbs,)*], #name, #opt_hack, #sql_down, #sql_up, #version ) })
      })
      .collect::<Vec<_>>();

    let mg_ident = format_ident!("{}", mg_name);
    let ms_ident = format_ident!("{}_MIGRATIONS", mg_name);

    let quote = quote! {
      const #mg_ident: oapth::MigrationGroupRef<'static> = oapth::MigrationGroupRef::new_ref(
        #mg_name, #mg_version
      );

      const #ms_ident: &[oapth::MigrationRef<'static, 'static>] = &[#(unsafe {
        let tuple = #filtered;
        oapth::Migration::new_ref(
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
    ts.extend(Into::<TokenStream>::into(quote));
  }

  let quote = quote! {
    const GROUPS: &[
      (oapth::MigrationGroupRef<'static>, &[oapth::MigrationRef<'static, 'static>])
    ] = &[
      #(#groups_and_migrations,)*
    ];
  };
  ts.extend(Into::<TokenStream>::into(quote));

  Ok(ts)
}

struct QuoteOption<T>(Option<T>);

impl<T> ToTokens for QuoteOption<T>
where
  T: ToTokens,
{
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    tokens.append_all(match self.0 {
      Some(ref t) => quote! { core::option::Option::Some(#t) },
      None => quote! { core::option::Option::None },
    });
  }
}
