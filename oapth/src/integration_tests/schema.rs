oapth_macros::_with_schema_! { pub(crate) mod with_schema; }
oapth_macros::_without_schema_! { pub(crate) mod without_schema; }

use crate::{integration_tests::AuxTestParams, BackEnd, Commands, MigrationGroup};
use std::path::Path;

pub(crate) async fn migrate_works<B>(c: &mut Commands<B>, aux: AuxTestParams, oapth_schema_tables: usize)
where
  B: BackEnd
{
  let path = Path::new("../oapth-test-utils/migrations.cfg");
  c.migrate_from_cfg(path).await.unwrap();
  let initial = MigrationGroup::new("initial", 1);
  let initial_migrations = c.back_end.migrations(initial).await.unwrap();
  assert_eq!(initial_migrations[0].checksum(), 12056372945923863254);
  assert_eq!(initial_migrations[0].version(), 1);
  assert_eq!(initial_migrations[0].name(), "create_author");
  assert_eq!(initial_migrations[1].version(), 2);
  assert_eq!(initial_migrations[1].name(), "create_post");
  assert_eq!(initial_migrations[2].version(), 3);
  assert_eq!(initial_migrations[2].name(), "insert_author");
  assert_eq!(initial_migrations[3].version(), 4);
  assert_eq!(initial_migrations[3].name(), "insert_post");
  assert_eq!(initial_migrations.get(4), None);
  let more_stuff = MigrationGroup::new("more_stuff", 2);
  let more_stuff_migrations = c.back_end.migrations(more_stuff).await.unwrap();
  assert_eq!(more_stuff_migrations[0].checksum(), 10291094225100056953);
  assert_eq!(more_stuff_migrations[0].version(), 1);
  assert_eq!(more_stuff_migrations[0].name(), "create_stuff");
  assert_eq!(more_stuff_migrations[1].version(), 2);
  assert_eq!(more_stuff_migrations[1].name(), "insert_stuff");
  assert_eq!(more_stuff_migrations.get(4), None);
  assert_eq!(
    c.back_end.tables(aux.default_schema).await.unwrap().len(),
    4 + aux.schema_regulator
  );
  assert_eq!(c.back_end.tables(aux.oapth_schema).await.unwrap().len(), oapth_schema_tables);
}
