use alloc::string::String;

/// Default environment variable
pub const DEFAULT_ENV_VAR: &str = "DATABASE_URL";

/// Configuration to connect to a database
#[derive(Debug)]
pub struct Config {
  url: String,
}

impl Config {
  /// Creates an instance with a given `url`.
  #[inline]
  pub fn with_url<I>(url: I) -> Self
  where
    I: Into<String>,
  {
    Self { url: url.into() }
  }

  /// Creates an instance with the contents of the default environment variable
  /// named `DATABASE_URL`.
  ///
  /// # Example
  ///
  /// ```rust
  /// # fn main() -> oapth::Result<()> {
  /// use oapth::Config;
  /// std::env::set_var("DATABASE_URL", "FOO");
  /// let _ = Config::with_url_from_default_var()?;
  /// # Ok(()) }
  /// ```
  #[cfg(feature = "std")]
  #[inline]
  pub fn with_url_from_default_var() -> crate::Result<Self> {
    Self::with_url_from_var(DEFAULT_ENV_VAR)
  }

  /// Creates an instance with the contents of the environment variable `env_var`.
  ///
  /// # Example
  ///
  /// ```rust
  /// # fn main() -> oapth::Result<()> {
  /// use oapth::Config;
  /// std::env::set_var("SOMETHING", "BAR");
  /// let _ = Config::with_url_from_var("SOMETHING")?;
  /// # Ok(()) }
  /// ```
  #[cfg(feature = "std")]
  #[inline]
  pub fn with_url_from_var(env_var: &str) -> crate::Result<Self> {
    let url = std::env::var(env_var).map_err(|_| crate::Error::MissingEnvVar)?;
    Ok(Self::with_url(url))
  }

  /// Url
  ///
  /// ```rust
  /// use oapth::Config;
  /// let c = Config::with_url("postgres://user_name:password@endpoint/database_name");
  /// assert_eq!(c.url(), "postgres://user_name:password@endpoint/database_name");
  /// ```
  #[inline]
  pub fn url(&self) -> &str {
    &self.url
  }
}
