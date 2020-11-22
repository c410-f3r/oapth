pub mod with_schema;
pub mod without_schema;

use crate::{integration_tests::AuxTestParams, BackEnd, Commands, MigrationGroup};
use std::path::Path;

async fn _migrate_works<B>(c: &mut Commands<B>, aux: AuxTestParams, oapth_schema_tables: usize)
where
  B: BackEnd,
{
  let path = Path::new("../oapth-test-utils/oapth.cfg");
  c.migrate_from_cfg(path, 128).await.unwrap();
  let initial = MigrationGroup::new(1, "initial");
  let initial_migrations = c.back_end.migrations(&initial).await.unwrap();
  assert_eq!(initial_migrations[0].checksum(), "11315267835087000498");
  assert_eq!(initial_migrations[0].version(), 1);
  assert_eq!(initial_migrations[0].name(), "create_author");
  assert_eq!(initial_migrations[1].checksum(), "12926561299591203746");
  assert_eq!(initial_migrations[1].version(), 2);
  assert_eq!(initial_migrations[1].name(), "create_post");
  assert_eq!(initial_migrations[2].checksum(), "11295713630394074471");
  assert_eq!(initial_migrations[2].version(), 3);
  assert_eq!(initial_migrations[2].name(), "insert_author");
  assert_eq!(initial_migrations[3].checksum(), "3532489065641291140");
  assert_eq!(initial_migrations[3].version(), 4);
  assert_eq!(initial_migrations[3].name(), "insert_post");
  assert_eq!(initial_migrations.get(4), None);
  let more_stuff = MigrationGroup::new(2, "more_stuff");
  let more_stuff_migrations = c.back_end.migrations(&more_stuff).await.unwrap();
  assert_eq!(more_stuff_migrations[0].checksum(), "11666382755144041358");
  assert_eq!(more_stuff_migrations[0].version(), 1);
  assert_eq!(more_stuff_migrations[0].name(), "create_stuff");
  assert_eq!(more_stuff_migrations.get(1), None);
  assert_eq!(
    c.back_end.all_tables(aux.default_schema).await.unwrap().len(),
    4 + aux.schema_regulator
  );
  assert_eq!(c.back_end.all_tables(aux.oapth_schema).await.unwrap().len(), oapth_schema_tables);
}
