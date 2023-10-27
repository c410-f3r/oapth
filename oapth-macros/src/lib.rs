//! Oapth macros

mod embed_migrations;

use proc_macro::{TokenStream, TokenTree};

/// Embed migrations
#[proc_macro]
pub fn embed_migrations(item: TokenStream) -> TokenStream {
  let err_tt = |s: &str| quote::quote!(compile_error!(#s)).into();

  macro_rules! manage_err {
    ($rslt:expr) => {
      match $rslt {
        Err(err) => {
          let s = err.to_string();
          return err_tt(&s);
        }
        Ok(elem) => elem,
      }
    };
  }

  let invalid_path_msg = "Please, provide a valid configuration path";
  let first_tt = manage_err!(item.into_iter().next().ok_or(invalid_path_msg));
  let TokenTree::Literal(literal) = first_tt else { return err_tt(invalid_path_msg) };
  let literal_string = literal.to_string();
  let literal_str_opt = || {
    let len_minus_one = literal_string.len().checked_sub(1)?;
    literal_string.get(1..len_minus_one)
  };
  let literal_str = manage_err!(literal_str_opt().ok_or(invalid_path_msg));
  manage_err!(embed_migrations::embed_migrations(literal_str))
}
