mod migrate;
#[cfg(feature = "std")]
mod migrate_from_dir;
#[cfg(feature = "dev-tools")]
mod reset;
mod rollback;
#[cfg(feature = "std")]
mod rollback_from_dir;
#[cfg(feature = "dev-tools")]
mod seed;
#[cfg(all(feature = "dev-tools", feature = "std"))]
mod seed_from_dir;
mod validate;
#[cfg(feature = "std")]
mod validate_from_dir;

use crate::Backend;

/// SQL commands facade
#[derive(Debug)]
pub struct Commands<B> {
  backend: B,
}

impl<B> Commands<B>
where
  B: Backend,
{
  /// Creates a new instance from a given backend.
  #[inline]
  pub fn new(backend: B) -> Self {
    Self { backend }
  }

  /// Returns the underlying backend
  #[inline]
  pub fn backend_mut(&mut self) -> &mut B {
    &mut self.backend
  }
}

#[cfg(all(feature = "_integration_tests", test))]
mod tests {
  use crate::{Backend, Commands, MigrationGroup};
  use std::path::Path;

  async fn migrate_and_rollback_works<B>(mut c: Commands<B>)
  where
    B: Backend,
  {
    let path = Path::new("../oapth-test-utils/migrations");
    c.migrate_from_dir(path, 128).await.unwrap();
    let initial = MigrationGroup::new(1, "initial");
    let initial_migrations = c.backend_mut().migrations(&initial).await.unwrap();
    assert_eq!(initial_migrations.len(), 4);
    assert_eq!(initial_migrations[0].checksum(), "11315267835087000498");
    assert_eq!(initial_migrations[0].version(), 1);
    assert_eq!(initial_migrations[0].name(), "create_author");
    let more_stuff = MigrationGroup::new(2, "more_stuff");
    let more_stuff_migrations = c.backend_mut().migrations(&more_stuff).await.unwrap();
    assert_eq!(more_stuff_migrations.len(), 1);
    assert_eq!(more_stuff_migrations[0].checksum(), "4849485378697205622");
    assert_eq!(more_stuff_migrations[0].version(), 1);
    assert_eq!(more_stuff_migrations[0].name(), "create_apple");
    c.rollback_from_dir(path, [0, 0].iter().cloned(), 128).await.unwrap();
    let initial = MigrationGroup::new(1, "initial");
    let initial_migrations = c.backend_mut().migrations(&initial).await.unwrap();
    assert_eq!(initial_migrations.len(), 0);
    let more_stuff = MigrationGroup::new(2, "more_stuff");
    let more_stuff_migrations = c.backend_mut().migrations(&more_stuff).await.unwrap();
    assert_eq!(more_stuff_migrations.len(), 0);
  }

  create_integration_tests!(migrate_and_rollback_works);
}
