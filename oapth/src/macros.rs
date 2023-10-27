macro_rules! create_enum {
    (
        $(#[$mac:meta])*
        $v:vis enum $enum_ident:ident {
            $($(#[$doc:meta])* $variant_ident:ident, $variant_str:literal;)*
        }
    ) => {
        $(#[$mac])*
        $v enum $enum_ident {
          $($(#[$doc])* $variant_ident,)*
        }

        impl $enum_ident {
            #[inline]
            /// Canonical string representation
            pub const fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$variant_ident => stringify!($variant_ident),)*
                }
            }

            #[allow(non_snake_case)]
            #[inline]
            /// The total number of variants
            pub const fn len() -> usize {
                let mut len = 0;
                $(
                    let $variant_ident = 1;
                    len += $variant_ident;
                )*
                len
            }
        }

        #[cfg(feature = "quote")]
        impl quote::ToTokens for $enum_ident {
          #[inline]
          fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
            match self {
                $(Self::$variant_ident => {
                    let ts = quote::quote!(oapth_commons::$enum_ident::$variant_ident);
                    tokens.extend(ts)
                },)*
            }
          }
        }

        impl core::fmt::Display for $enum_ident {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}", self.as_str())
            }
        }

        impl core::str::FromStr for $enum_ident {
            type Err = crate::Error;

            #[inline]
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(match s {
                    $($variant_str => Self::$variant_ident,)*
                    _ => return Err(crate::Error::IncompleteSqlFile),
                })
            }
        }
    }
}
