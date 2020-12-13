#[oapth_macros::dev_tools_]
pub async fn clean_drops_all_objs<B>(
  c: &mut crate::Commands<B>,
  aux: crate::integration_tests::AuxTestParams,
) where
  B: crate::BackEnd
{
  crate::integration_tests::create_foo_table(c, aux.default_schema_prefix).await;
  assert_eq!(c.back_end.tables(aux.default_schema).await.unwrap().len(), 1);
  c.back_end.execute("CREATE SCHEMA bar").await.unwrap();
  crate::integration_tests::create_foo_table(c, "bar.").await;
  assert_eq!(c.back_end.tables("bar").await.unwrap().len(), 1);
  c.back_end.execute("CREATE VIEW view AS SELECT * FROM foo WHERE id = 1;").await.unwrap();
  c.back_end.execute("CREATE TYPE mood AS ENUM ('sad', 'ok', 'happy');").await.unwrap();
  c.back_end.execute("CREATE DOMAIN integer0 AS INTEGER CONSTRAINT must_be_greater_than_or_equal_to_zero_chk CHECK(VALUE >= 0);").await.unwrap();
  c.back_end.execute("CREATE FUNCTION time_subtype_diff(x time, y time) RETURNS float8 AS 'SELECT EXTRACT(EPOCH FROM (x - y))' LANGUAGE sql STRICT IMMUTABLE").await.unwrap();
  c.back_end.execute("CREATE PROCEDURE something() LANGUAGE SQL AS $$ $$").await.unwrap();
  c.back_end.execute("CREATE TYPE A_TYPE AS (field INTEGER[31])").await.unwrap();
  c.back_end.execute("CREATE SEQUENCE serial START 101;").await.unwrap();

  assert_eq!(crate::fixed_sql_commands::pg::schemas(&mut c.back_end).await.unwrap().len(), 1);
  assert_eq!(c.back_end.tables(aux.default_schema).await.unwrap().len(), 1);
  assert_eq!(crate::fixed_sql_commands::pg::views(&mut c.back_end, "public").await.unwrap().len(), 1);
  assert_eq!(crate::fixed_sql_commands::pg::domains(&mut c.back_end, "public").await.unwrap().len(), 1);
  assert_eq!(crate::fixed_sql_commands::pg::functions(&mut c.back_end, "public").await.unwrap().len(), 1);
  assert_eq!(crate::fixed_sql_commands::pg::procedures(&mut c.back_end, "public").await.unwrap().len(), 1);
  assert_eq!(crate::fixed_sql_commands::pg::types(&mut c.back_end, "public").await.unwrap().len(), 1);
  assert_eq!(crate::fixed_sql_commands::pg::sequences(&mut c.back_end, "public").await.unwrap().len(), 1);
  assert_eq!(crate::fixed_sql_commands::pg::enums(&mut c.back_end, "public").await.unwrap().len(), 1);

  c.clean().await.unwrap();

  assert_eq!(crate::fixed_sql_commands::pg::schemas(&mut c.back_end).await.unwrap().len(), 0);
  assert_eq!(c.back_end.tables(aux.default_schema).await.unwrap().len(), 0);
  assert_eq!(crate::fixed_sql_commands::pg::domains(&mut c.back_end, "public").await.unwrap().len(), 0);
  assert_eq!(crate::fixed_sql_commands::pg::functions(&mut c.back_end, "public").await.unwrap().len(), 0);
  assert_eq!(crate::fixed_sql_commands::pg::procedures(&mut c.back_end, "public").await.unwrap().len(), 0);
  assert_eq!(crate::fixed_sql_commands::pg::types(&mut c.back_end, "public").await.unwrap().len(), 0);
  assert_eq!(crate::fixed_sql_commands::pg::sequences(&mut c.back_end, "public").await.unwrap().len(), 0);
  assert_eq!(crate::fixed_sql_commands::pg::views(&mut c.back_end, "public").await.unwrap().len(), 0);
  assert_eq!(crate::fixed_sql_commands::pg::enums(&mut c.back_end, "public").await.unwrap().len(), 0);

}
