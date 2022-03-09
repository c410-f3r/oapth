#[oapth_macros::_dev_tools]
pub(crate) async fn clean_drops_all_objs<B>(
  buffer: &mut String,
  c: &mut crate::Commands<B, >,
  _: crate::integration_tests::AuxTestParams,
) where
  B: crate::Backend
{
  crate::integration_tests::create_foo_table(c, "public.").await;
  c.backend.execute("CREATE SCHEMA bar").await.unwrap();
  crate::integration_tests::create_foo_table(c, "bar.").await;
  c.backend.execute("CREATE DOMAIN integer0 AS INTEGER CONSTRAINT must_be_greater_than_or_equal_to_zero_chk CHECK(VALUE >= 0);").await.unwrap();
  c.backend.execute("CREATE FUNCTION time_subtype_diff(x time, y time) RETURNS float8 AS 'SELECT EXTRACT(EPOCH FROM (x - y))' LANGUAGE sql STRICT IMMUTABLE").await.unwrap();
  c.backend.execute("CREATE PROCEDURE something() LANGUAGE SQL AS $$ $$").await.unwrap();
  c.backend.execute("CREATE SEQUENCE serial START 101;").await.unwrap();
  c.backend.execute("CREATE TYPE a_type AS (field INTEGER[31])").await.unwrap();
  c.backend.execute("CREATE TYPE mood AS ENUM ('sad', 'ok', 'happy');").await.unwrap();
  c.backend.execute("CREATE VIEW view AS SELECT * FROM foo WHERE id = 1;").await.unwrap();
  
  assert_eq!(c.backend.tables("public").await.unwrap().len(), 1);
  assert_eq!(crate::fixed_sql_commands::pg::schemas(&mut c.backend).await.unwrap().len(), 1);
  assert_eq!(c.backend.tables("bar").await.unwrap().len(), 1);
  assert_eq!(crate::fixed_sql_commands::pg::domains(&mut c.backend, buffer, "public").await.unwrap().len(), 1);
  assert_eq!(crate::fixed_sql_commands::pg::enums(&mut c.backend, buffer, "public").await.unwrap().len(), 1);
  assert_eq!(crate::fixed_sql_commands::pg::functions(&mut c.backend, buffer, "public").await.unwrap().len(), 1);
  assert_eq!(crate::fixed_sql_commands::pg::procedures(&mut c.backend, buffer, "public").await.unwrap().len(), 1);
  assert_eq!(crate::fixed_sql_commands::pg::sequences(&mut c.backend, buffer, "public").await.unwrap().len(), 1);
  assert_eq!(crate::fixed_sql_commands::pg::types(&mut c.backend, buffer, "public").await.unwrap().len(), 2);
  assert_eq!(crate::fixed_sql_commands::pg::views(&mut c.backend, buffer, "public").await.unwrap().len(), 1);

  c.clean(buffer).await.unwrap();

  assert_eq!(c.backend.tables("public").await.unwrap().len(), 0);
  assert_eq!(crate::fixed_sql_commands::pg::schemas(&mut c.backend).await.unwrap().len(), 0);
  assert_eq!(c.backend.tables("bar").await.unwrap().len(), 0);
  assert_eq!(crate::fixed_sql_commands::pg::domains(&mut c.backend, buffer, "public").await.unwrap().len(), 0);
  assert_eq!(crate::fixed_sql_commands::pg::enums(&mut c.backend, buffer, "public").await.unwrap().len(), 0);
  assert_eq!(crate::fixed_sql_commands::pg::functions(&mut c.backend, buffer, "public").await.unwrap().len(), 0);
  assert_eq!(crate::fixed_sql_commands::pg::procedures(&mut c.backend, buffer, "public").await.unwrap().len(), 0);
  assert_eq!(crate::fixed_sql_commands::pg::sequences(&mut c.backend, buffer, "public").await.unwrap().len(), 0);
  assert_eq!(crate::fixed_sql_commands::pg::types(&mut c.backend, buffer, "public").await.unwrap().len(), 0);
  assert_eq!(crate::fixed_sql_commands::pg::views(&mut c.backend, buffer, "public").await.unwrap().len(), 0);
}
