use crate::{integration_tests::AuxTestParams, BackEnd, Commands, MigrationGroup};
use std::path::Path;

pub(crate) async fn all_tables_returns_the_number_of_tables_of_the_default_schema<B>(
  c: &mut Commands<B>,
  aux: AuxTestParams,
) where
  B: BackEnd,
{
  c.back_end.execute("CREATE TABLE foo(id INT)").await.unwrap();
  assert_eq!(c.back_end.tables(aux.default_schema).await.unwrap().len(), 1);
}

pub(crate) async fn rollback_works<B>(c: &mut Commands<B>, aux: AuxTestParams)
where
  B: BackEnd
{
  let path = Path::new("../oapth-test-utils/migrations.cfg");
  c.migrate_from_cfg(path).await.unwrap();
  c.rollback_from_cfg(path, &[0, 0][..]).await.unwrap();
  let initial = MigrationGroup::new("initial", 1);
  let initial_migrations = c.back_end.migrations(initial).await.unwrap();
  assert_eq!(initial_migrations.len(), 0);
  let more_stuff = MigrationGroup::new("more_stuff", 2);
  let more_stuff_migrations = c.back_end.migrations(more_stuff).await.unwrap();
  assert_eq!(more_stuff_migrations.len(), 0);
  assert_eq!(c.back_end.tables(aux.default_schema).await.unwrap().len(), aux.schema_regulator);
  assert_eq!(c.back_end.tables(aux.oapth_schema).await.unwrap().len(), 2);
}
