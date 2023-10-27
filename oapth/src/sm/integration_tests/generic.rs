use crate::{
  sm::{integration_tests::AuxTestParams, Commands, DbMigration, MigrationGroup, SchemaManagement},
  Identifier,
};
use std::path::Path;

pub(crate) async fn all_tables_returns_the_number_of_tables_of_the_default_schema<D>(
  (buffer_cmd, _, buffer_idents): (&mut String, &mut Vec<DbMigration>, &mut Vec<Identifier>),
  c: &mut Commands<D>,
  aux: AuxTestParams,
) where
  D: SchemaManagement,
{
  c.database.execute("CREATE TABLE foo(id INT)").await.unwrap();
  c.database.table_names(buffer_cmd, buffer_idents, aux.default_schema).await.unwrap();
  assert_eq!(buffer_idents.len(), 1);
  buffer_idents.clear();
}

pub(crate) async fn rollback_works<D>(
  (buffer_cmd, buffer_db_migrations, buffer_idents): (
    &mut String,
    &mut Vec<DbMigration>,
    &mut Vec<Identifier>,
  ),
  c: &mut Commands<D>,
  aux: AuxTestParams,
) where
  D: SchemaManagement,
{
  let path = Path::new("../.test-utils/migrations.toml");
  c.migrate_from_toml_path((buffer_cmd, buffer_db_migrations), path).await.unwrap();
  c.rollback_from_toml((buffer_cmd, buffer_db_migrations), path, &[0, 0][..]).await.unwrap();
  let initial = MigrationGroup::new("initial", 1);
  let more_stuff = MigrationGroup::new("more_stuff", 2);

  c.database.migrations(buffer_cmd, &initial, buffer_db_migrations).await.unwrap();
  assert_eq!(buffer_db_migrations.len(), 0);

  c.database.migrations(buffer_cmd, &more_stuff, buffer_db_migrations).await.unwrap();
  assert_eq!(buffer_db_migrations.len(), 0);

  c.database.table_names(buffer_cmd, buffer_idents, aux.default_schema).await.unwrap();
  assert_eq!(buffer_idents.len(), aux.schema_regulator);
  buffer_idents.clear();

  c.database.table_names(buffer_cmd, buffer_idents, aux.oapth_schema).await.unwrap();
  assert_eq!(buffer_idents.len(), 2);
  buffer_idents.clear();
}
