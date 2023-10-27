#[cfg(feature = "sm-dev")]
use crate::sm::fixed_sql_commands::postgres;

#[cfg(feature = "sm-dev")]
pub(crate) async fn _clean_drops_all_objs<D>(
  (buffer_cmd, _, buffer_idents): (
    &mut String,
    &mut Vec<crate::sm::DbMigration>,
    &mut Vec<crate::Identifier>,
  ),
  c: &mut crate::sm::Commands<D>,
  _: crate::sm::integration_tests::AuxTestParams,
) where
  D: crate::sm::SchemaManagement,
{
  crate::sm::integration_tests::create_foo_table(buffer_cmd, c, "public.").await;
  c.database.execute("CREATE SCHEMA bar").await.unwrap();
  crate::sm::integration_tests::create_foo_table(buffer_cmd, c, "bar.").await;
  c.database.execute("CREATE DOMAIN integer0 AS INTEGER CONSTRAINT must_be_greater_than_or_equal_to_zero_chk CHECK(VALUE >= 0);").await.unwrap();
  c.database.execute("CREATE FUNCTION time_subtype_diff(x time, y time) RETURNS float8 AS 'SELECT EXTRACT(EPOCH FROM (x - y))' LANGUAGE sql STRICT IMMUTABLE").await.unwrap();
  c.database.execute("CREATE PROCEDURE something() LANGUAGE SQL AS $$ $$").await.unwrap();
  c.database.execute("CREATE SEQUENCE serial START 101;").await.unwrap();
  c.database.execute("CREATE TYPE a_type AS (field INTEGER[31])").await.unwrap();
  c.database.execute("CREATE TYPE mood AS ENUM ('sad', 'ok', 'happy');").await.unwrap();
  c.database.execute("CREATE VIEW view AS SELECT * FROM foo WHERE id = 1;").await.unwrap();

  c.database.table_names(buffer_cmd, buffer_idents, "public").await.unwrap();
  assert_eq!(buffer_idents.len(), 1);
  buffer_idents.clear();

  postgres::_schemas(&mut c.database, buffer_idents).await.unwrap();
  assert_eq!(buffer_idents.len(), 1);
  buffer_idents.clear();

  c.database.table_names(buffer_cmd, buffer_idents, "bar").await.unwrap();
  assert_eq!(buffer_idents.len(), 1);
  buffer_idents.clear();

  postgres::_domains(&mut c.database, buffer_idents).await.unwrap();
  assert_eq!(buffer_idents.len(), 1);
  buffer_idents.clear();

  postgres::_enums(&mut c.database, buffer_idents).await.unwrap();
  assert_eq!(buffer_idents.len(), 1);
  buffer_idents.clear();

  postgres::_pg_proc((buffer_cmd, buffer_idents), &mut c.database, 'f').await.unwrap();
  assert_eq!(buffer_idents.len(), 1);
  buffer_idents.clear();

  postgres::_pg_proc((buffer_cmd, buffer_idents), &mut c.database, 'p').await.unwrap();
  assert_eq!(buffer_idents.len(), 1);
  buffer_idents.clear();

  postgres::_sequences(&mut c.database, buffer_idents).await.unwrap();
  assert_eq!(buffer_idents.len(), 1);
  buffer_idents.clear();

  postgres::_types(&mut c.database, buffer_idents).await.unwrap();
  assert_eq!(buffer_idents.len(), 2);
  buffer_idents.clear();

  postgres::_views(&mut c.database, buffer_idents).await.unwrap();
  assert_eq!(buffer_idents.len(), 1);
  buffer_idents.clear();

  c.clear((buffer_cmd, buffer_idents)).await.unwrap();

  c.database.table_names(buffer_cmd, buffer_idents, "public").await.unwrap();
  assert_eq!(buffer_idents.len(), 0);
  buffer_idents.clear();

  postgres::_schemas(&mut c.database, buffer_idents).await.unwrap();
  assert_eq!(buffer_idents.len(), 0);
  buffer_idents.clear();

  c.database.table_names(buffer_cmd, buffer_idents, "bar").await.unwrap();
  assert_eq!(buffer_idents.len(), 0);
  buffer_idents.clear();

  postgres::_domains(&mut c.database, buffer_idents).await.unwrap();
  assert_eq!(buffer_idents.len(), 0);
  buffer_idents.clear();

  postgres::_enums(&mut c.database, buffer_idents).await.unwrap();
  assert_eq!(buffer_idents.len(), 0);
  buffer_idents.clear();

  postgres::_pg_proc((buffer_cmd, buffer_idents), &mut c.database, 'f').await.unwrap();
  assert_eq!(buffer_idents.len(), 0);
  buffer_idents.clear();

  postgres::_pg_proc((buffer_cmd, buffer_idents), &mut c.database, 'p').await.unwrap();
  assert_eq!(buffer_idents.len(), 0);
  buffer_idents.clear();

  postgres::_sequences(&mut c.database, buffer_idents).await.unwrap();
  assert_eq!(buffer_idents.len(), 0);
  buffer_idents.clear();

  postgres::_types(&mut c.database, buffer_idents).await.unwrap();
  assert_eq!(buffer_idents.len(), 0);
  buffer_idents.clear();

  postgres::_views(&mut c.database, buffer_idents).await.unwrap();
  assert_eq!(buffer_idents.len(), 0);
  buffer_idents.clear();
}
