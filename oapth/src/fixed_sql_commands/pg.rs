use arrayvec::ArrayString;
use core::fmt::Write;

pub(crate) const CREATE_MIGRATION_TABLES: &str = concat!(
  "CREATE SCHEMA IF NOT EXISTS _oapth; \
  CREATE TABLE IF NOT EXISTS _oapth._oapth_migration_group (",
  oapth_migration_group_columns!(),
  ");
  CREATE TABLE IF NOT EXISTS _oapth._oapth_migration (",
  serial_id!(),
  "created_on TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,",
  oapth_migration_columns!(),
  ");"
);

#[inline]
#[oapth_macros::_dev_tools]
pub(crate) async fn clean<B>(back_end: &mut B) -> crate::Result<()>
where
  B: crate::BackEnd,
{
  let mut buffer: ArrayString<2048> = ArrayString::new();

  for schema in schemas(back_end).await? {
    buffer.write_fmt(format_args!("DROP SCHEMA \"{}\" CASCADE;", schema))?;
  }

  for domain in domains(back_end, "public").await? {
    buffer.write_fmt(format_args!("DROP DOMAIN \"{}\" CASCADE;", domain))?;
  }

  for function in functions(back_end, "public").await? {
    buffer.write_fmt(format_args!("DROP FUNCTION \"{}\" CASCADE;", function))?;
  }

  for view in views(back_end, "public").await? {
    buffer.write_fmt(format_args!("DROP VIEW \"{}\" CASCADE;", view))?;
  }

  for table in back_end.tables("public").await? {
    buffer.write_fmt(format_args!("DROP TABLE \"{}\" CASCADE;", table))?;
  }

  for procedure in procedures(back_end, "public").await? {
    buffer.write_fmt(format_args!("DROP PROCEDURE \"{}\" CASCADE;", procedure))?;
  }

  for ty in types(back_end, "public").await? {
    buffer.write_fmt(format_args!("DROP TYPE \"{}\" CASCADE;", ty))?;
  }

  for sequence in sequences(back_end, "public").await? {
    buffer.write_fmt(format_args!("DROP SEQUENCE \"{}\" CASCADE;", sequence))?;
  }

  back_end.execute(&buffer).await?;

  Ok(())
}

// https://github.com/flyway/flyway/blob/master/flyway-core/src/main/java/org/flywaydb/core/internal/database/postgresql/PostgreSQLSchema.java
#[cfg(test)]
#[inline]
#[oapth_macros::_dev_tools]
pub(crate) async fn enums<B>(back_end: &mut B, schema: & str) -> crate::Result<Vec<String>>
where
  B: crate::BackEnd
{
  let mut buffer = ArrayString::<512>::new();
  buffer.write_fmt(format_args!(
    "SELECT
      t.typname AS generic_column
    FROM
      pg_catalog.pg_type t
      INNER JOIN pg_catalog.pg_namespace n ON n.oid = t.typnamespace
    WHERE
      n.nspname = '{schema}' AND  t.typtype = 'e'
    ",
    schema = schema
  ))?;
  Ok(back_end.query_string(&buffer).await?)
}

// https://github.com/flyway/flyway/blob/master/flyway-core/src/main/java/org/flywaydb/core/internal/database/postgresql/PostgreSQLSchema.java
#[inline]
#[oapth_macros::_dev_tools]
pub(crate) async fn views<B>(back_end: &mut B, schema: & str) -> crate::Result<Vec<String>>
where
 B: crate::BackEnd,
{
  let mut buffer = ArrayString::<512>::new();
  buffer.write_fmt(format_args!(
    "
    SELECT
      relname AS generic_column
    FROM pg_catalog.pg_class c
      JOIN pg_namespace n ON n.oid = c.relnamespace
      LEFT JOIN pg_depend dep ON dep.objid = c.oid AND dep.deptype = 'e'
    WHERE c.relkind = 'v'
      AND  n.nspname = '{schema}'
      AND dep.objid IS NULL
    ",
    schema = schema
  ))?;
  Ok(back_end.query_string(&buffer).await?)
}

// https://github.com/flyway/flyway/blob/master/flyway-core/src/main/java/org/flywaydb/core/internal/database/postgresql/PostgreSQLSchema.java
#[inline]
#[oapth_macros::_dev_tools]
pub(crate) async fn sequences<B>(back_end: &mut B, schema: & str) -> crate::Result<Vec<String>>
where
  B: crate::BackEnd,
{
  let mut buffer = ArrayString::<256>::new();
  buffer.write_fmt(format_args!(
    "SELECT
      sequence_name AS generic_column
    FROM
      information_schema.sequences
    WHERE
      sequence_schema = '{schema}'",
    schema = schema
  ))?;
  Ok(back_end.query_string(&buffer).await?)
}

// https://github.com/flyway/flyway/blob/master/flyway-core/src/main/java/org/flywaydb/core/internal/database/postgresql/PostgreSQLSchema.java
#[oapth_macros::_dev_tools]
#[inline]
pub(crate) async fn domains<B>(back_end: &mut B, schema: &str) -> crate::Result<Vec<String>>
where
  B: crate::BackEnd,
{
  let mut buffer = ArrayString::<512>::new();
  buffer.write_fmt(format_args!(
    "
    SELECT
      t.typname AS generic_column
    FROM pg_catalog.pg_type t
      LEFT JOIN pg_catalog.pg_namespace n ON n.oid = t.typnamespace
      LEFT JOIN pg_depend dep ON dep.objid = t.oid AND dep.deptype = 'e'
    WHERE t.typtype = 'd'
      AND n.nspname = '{schema}'
      AND dep.objid IS NULL
    ",
    schema = schema
  ))?;
  Ok(back_end.query_string(&buffer).await?)
}

#[inline]
#[oapth_macros::_dev_tools]
pub(crate) async fn functions<B>(back_end: &mut B, schema: &str) -> crate::Result<Vec<String>>
where
  B: crate::BackEnd,
{
  Ok(back_end.query_string(&pg_proc('f', schema)?).await?)
}

#[inline]
#[oapth_macros::_dev_tools]
pub(crate) async fn procedures<B>(back_end: &mut B, schema: &str) -> crate::Result<Vec<String>>
where
  B: crate::BackEnd,
{
  Ok(back_end.query_string(&pg_proc('p', schema)?).await?)
}

#[inline]
#[oapth_macros::_dev_tools]
pub(crate) async fn schemas<B>(back_end: &mut B) -> crate::Result<Vec<String>>
where
  B: crate::BackEnd,
{
  Ok(back_end.query_string("SELECT
    pc_ns.nspname AS generic_column
  FROM
    pg_catalog.pg_namespace pc_ns
  WHERE
    nspname NOT IN ('information_schema', 'pg_catalog', 'public')
    AND nspname NOT LIKE 'pg_toast%'
    AND nspname NOT LIKE 'pg_temp_%'
  ").await?)
}

// https://github.com/flyway/flyway/blob/master/flyway-core/src/main/java/org/flywaydb/core/internal/database/postgresql/PostgreSQLSchema.java
#[inline]
pub(crate) fn tables(schema: &str) -> crate::Result<ArrayString<1024>> {
  let mut buffer = ArrayString::new();
  buffer.write_fmt(
    format_args!(
      "
        SELECT
        tables.table_name AS generic_column
        FROM
          information_schema.tables tables
          -- that don't depend on an extension
          LEFT JOIN pg_depend dep ON dep.objid = (quote_ident(tables.table_schema)||'.'||quote_ident(tables.table_name))::regclass::oid AND dep.deptype = 'e'
        WHERE
          -- in this schema
          table_schema = '{schema}'
          -- that are real tables (as opposed to views)
          AND table_type='BASE TABLE'
          -- with no extension depending on them
          AND dep.objid IS NULL
          -- and are not child tables (= do not inherit from another table).
          AND NOT (
            SELECT EXISTS (SELECT inhrelid FROM pg_catalog.pg_inherits
            WHERE inhrelid = (quote_ident(tables.table_schema)||'.'||quote_ident(tables.table_name))::regclass::oid)
          )
      ",
      schema = schema
    )
  )?;
  Ok(buffer)
}

// https://github.com/flyway/flyway/blob/master/flyway-core/src/main/java/org/flywaydb/core/internal/database/postgresql/PostgreSQLSchema.java
#[oapth_macros::_dev_tools]
#[inline]
pub(crate) async fn types<B>(back_end: &mut B, schema: &str) -> crate::Result<Vec<String>>
where
  B: crate::BackEnd,
{
  let mut buffer = ArrayString::<1024>::new();
  buffer.write_fmt(format_args!(
    "SELECT
      typname AS generic_column
    FROM
      pg_catalog.pg_type t
      LEFT JOIN pg_depend dep ON dep.objid = t.oid and dep.deptype = 'e'
    WHERE
      (t.typrelid = 0 OR (
        SELECT c.relkind = 'c' FROM pg_catalog.pg_class c WHERE c.oid = t.typrelid)
      )
      AND NOT EXISTS(
        SELECT 1 FROM pg_catalog.pg_type el WHERE el.oid = t.typelem AND el.typarray = t.oid
      )
      AND t.typnamespace in (
        select oid from pg_catalog.pg_namespace where nspname = '{schema}'
      )
      AND dep.objid is null
      AND t.typtype != 'd'",
      schema = schema
  ))?;
  Ok(back_end.query_string(&buffer).await?)
}

// https://github.com/flyway/flyway/blob/master/flyway-core/src/main/java/org/flywaydb/core/internal/database/postgresql/PostgreSQLSchema.java
#[oapth_macros::_dev_tools]
#[inline]
fn pg_proc(prokind: char, schema: &str) -> crate::Result<ArrayString<512>>
{
  let mut buffer = ArrayString::new();
  buffer.write_fmt(format_args!(
    "
    SELECT
      proname AS generic_column
    FROM
      pg_proc
      INNER JOIN pg_namespace ns ON (pg_proc.pronamespace = ns.oid)
      -- that don't depend on an extension
      LEFT JOIN pg_depend dep ON dep.objid = pg_proc.oid AND dep.deptype = 'e'
    WHERE
      ns.nspname = '{schema}'
      AND dep.objid IS NULL
      AND pg_proc.prokind = '{prokind}'
    ",
    prokind = prokind,
    schema = schema
  ))?;
  Ok(buffer)
}
