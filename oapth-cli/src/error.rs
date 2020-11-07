use core::fmt;

pub enum Error {
  Oapth(oapth::Error),
}

impl From<oapth::Error> for Error {
  #[inline]
  fn from(from: oapth::Error) -> Self {
    Self::Oapth(from)
  }
}

impl fmt::Debug for Error {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match *self {
      Self::Oapth(ref e) => write!(f, "oapth: {}", e),
    }
  }
}

impl fmt::Display for Error {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Debug::fmt(self, f)
  }
}

impl std::error::Error for Error {}
