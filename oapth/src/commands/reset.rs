use crate::{Backend, Commands};
use arrayvec::ArrayString;
use core::fmt::Write;

impl<B> Commands<B>
where
  B: Backend,
{
  /// If existing, drops a given database and then re-creates it again.
  #[inline]
  pub async fn reset<'a>(&'a mut self, name: &'a str) -> crate::Result<()> {
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
