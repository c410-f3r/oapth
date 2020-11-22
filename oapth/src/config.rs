use alloc::string::String;

#[cfg(feature = "std")]
const DEFAULT_ENV_VAR: &str = "DATABASE_URL";

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

  /// Database type
  ///
  /// ```rust
  /// use oapth::Config;
  /// let c = Config::with_url("postgres://user_name:password@endpoint:1234/database_name");
  /// assert_eq!(c.database().unwrap(), "postgres");
  /// ```
  #[inline]
  pub fn database(&self) -> crate::Result<&str> {
    let map = || crate::Error::Other("Invalid URL - Port is not an integer");
    self.url.split(':').next().ok_or_else(map)
  }

  /// Host with optional port
  ///
  /// ```rust
  /// use oapth::Config;
  /// let c0 = Config::with_url("postgres://user_name:password@endpoint/database_name");
  /// assert_eq!(c0.full_host().unwrap(), "endpoint");
  /// let c1 = Config::with_url("postgres://user_name:password@endpoint:1234/database_name");
  /// assert_eq!(c1.full_host().unwrap(), "endpoint:1234");
  /// ```
  #[inline]
  pub fn full_host(&self) -> crate::Result<&str> {
    let opt = || {
      let last_slash = self.url.rfind('/')?;
      let s0 = self.url.get(..last_slash)?;
      let last_at = s0.rfind('@')?;
      s0.get(last_at.saturating_add(1)..)
    };
    opt().ok_or(crate::Error::Other("Invalid URL - Missing host with optional port"))
  }

  /// Host
  ///
  /// ```rust
  /// use oapth::Config;
  /// let c0 = Config::with_url("postgres://user_name:password@endpoint/database_name");
  /// assert_eq!(c0.host().unwrap(), "endpoint");
  /// let c1 = Config::with_url("postgres://user_name:password@endpoint:1234/database_name");
  /// assert_eq!(c1.host().unwrap(), "endpoint");
  /// ```
  #[inline]
  pub fn host(&self) -> crate::Result<&str> {
    let map = || crate::Error::Other("Invalid URL - Missing database port");
    let full_host = self.full_host()?;
    if let Some(rslt) = full_host.find(':') {
      full_host.get(..rslt).ok_or_else(map)
    } else {
      Ok(full_host)
    }
  }

  /// Name
  ///
  /// ```rust
  /// use oapth::Config;
  /// let c = Config::with_url("postgres://user_name:password@endpoint/database_name");
  /// assert_eq!(c.name().unwrap(), "database_name");
  /// ```
  #[inline]
  pub fn name(&self) -> crate::Result<&str> {
    let opt = || {
      let last_slash = self.url.rfind('/')?;
      self.url.get(last_slash.saturating_add(1)..)
    };
    opt().ok_or(crate::Error::Other("Invalid URL - Missing database name"))
  }

  /// Password
  ///
  /// ```rust
  /// use oapth::Config;
  /// let c = Config::with_url("postgres://user_name:password@endpoint:1234/database_name");
  /// assert_eq!(c.password().unwrap(), "password");
  /// ```
  #[inline]
  pub fn password(&self) -> crate::Result<&str> {
    let opt = || {
      let with_password = self.url.split(':').nth(2)?;
      let at_idx = with_password.find('@')?;
      with_password.get(0..at_idx)
    };
    opt().ok_or(crate::Error::Other("Invalid URL - Missing database password"))
  }

  /// Port
  ///
  /// ```rust
  /// use oapth::Config;
  /// let c = Config::with_url("postgres://user_name:password@endpoint:1234/database_name");
  /// assert_eq!(c.port().unwrap(), 1234);
  /// ```
  #[inline]
  pub fn port(&self) -> crate::Result<u16> {
    let map = || crate::Error::Other("Invalid URL - Missing database port");
    let full_host = self.full_host()?;
    let last_colon = full_host.rfind(':').ok_or_else(map)?;
    let s = full_host.get(last_colon.saturating_add(1)..).ok_or_else(map)?;
    s.parse().map_err(|_| crate::Error::Other("Invalid URL - Port is not an integer"))
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

  /// User
  ///
  /// ```rust
  /// use oapth::Config;
  /// let c = Config::with_url("postgres://user_name:password@endpoint:1234/database_name");
  /// assert_eq!(c.user().unwrap(), "user_name");
  /// ```
  #[inline]
  pub fn user(&self) -> crate::Result<&str> {
    let opt = || self.url.split(':').nth(1)?.get(2..);
    opt().ok_or(crate::Error::Other("Invalid URL - Missing database password"))
  }
}
