use crate::{Backend, Commands};
use arrayvec::ArrayString;
use core::fmt::Write;

impl<B> Commands<B>
where
  B: Backend,
{
  /// Attempts to drop a given database and then recreate it again.
  #[inline]
  pub async fn recreate<'a>(&'a mut self, name: &'a str) -> crate::Result<()> {
    let mut buffer = ArrayString::<[u8; 128]>::new();
    buffer.write_fmt(format_args!(
      "
      DROP DATABASE {name};
      CREATE DATABASE {name};
      ",
      name = name
    ))?;
    self.backend.execute(&buffer).await?;
    Ok(())
  }
}
