use crate::{database::Database, Identifier};
use alloc::{string::String, vec::Vec};
use core::fmt::Write;

pub(crate) const _CREATE_MIGRATION_TABLES: &str = concat!(
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
pub(crate) async fn _clear<D>(
  (buffer_cmd, buffer_idents): (&mut String, &mut Vec<Identifier>),
  db: &mut D,
) -> crate::Result<()>
where
  D: Database,
{
  _schemas(db, buffer_idents).await?;
  _push_drop((buffer_cmd, buffer_idents), "SCHEMA")?;

  _domains(db, buffer_idents).await?;
  _push_drop((buffer_cmd, buffer_idents), "DOMAIN")?;

  _pg_proc((buffer_cmd, buffer_idents), db, 'f').await?;
  _push_drop((buffer_cmd, buffer_idents), "FUNCTION")?;

  _views(db, buffer_idents).await?;
  _push_drop((buffer_cmd, buffer_idents), "VIEW")?;

  _table_names(buffer_cmd, db, buffer_idents, "public").await?;
  _push_drop((buffer_cmd, buffer_idents), "TABLE")?;

  _pg_proc((buffer_cmd, buffer_idents), db, 'p').await?;
  _push_drop((buffer_cmd, buffer_idents), "PROCEDURE")?;

  _types(db, buffer_idents).await?;
  _push_drop((buffer_cmd, buffer_idents), "TYPE")?;

  _sequences(db, buffer_idents).await?;
  _push_drop((buffer_cmd, buffer_idents), "SEQUENCE")?;

  db.execute(buffer_cmd).await?;
  buffer_cmd.clear();

  Ok(())
}

#[inline]
pub(crate) async fn _domains<D>(db: &mut D, results: &mut Vec<Identifier>) -> crate::Result<()>
where
  D: Database,
{
  db.simple_entities(
    "
    SELECT
      t.typname AS generic_column
    FROM pg_catalog.pg_type t
      LEFT JOIN pg_catalog.pg_namespace n ON n.oid = t.typnamespace
      LEFT JOIN pg_depend dep ON dep.objid = t.oid AND dep.deptype = 'e'
    WHERE t.typtype = 'd'
      AND n.nspname = 'public'
      AND dep.objid IS NULL
    ",
    results,
  )
  .await?;
  Ok(())
}

#[cfg(test)]
#[inline]
pub(crate) async fn _enums<D>(db: &mut D, results: &mut Vec<Identifier>) -> crate::Result<()>
where
  D: Database,
{
  db.simple_entities(
    "SELECT
      t.typname AS generic_column
    FROM
      pg_catalog.pg_type t
      INNER JOIN pg_catalog.pg_namespace n ON n.oid = t.typnamespace
    WHERE
      n.nspname = 'public' AND  t.typtype = 'e'
    ",
    results,
  )
  .await
}

#[inline]
pub(crate) async fn _pg_proc<D>(
  (buffer_cmd, buffer_idents): (&mut String, &mut Vec<Identifier>),
  db: &mut D,
  prokind: char,
) -> crate::Result<()>
where
  D: Database,
{
  let before = buffer_cmd.len();
  buffer_cmd.write_fmt(format_args!(
    "
    SELECT
      proname AS generic_column
    FROM
      pg_proc
      INNER JOIN pg_namespace ns ON (pg_proc.pronamespace = ns.oid)
      -- that don't depend on an extension
      LEFT JOIN pg_depend dep ON dep.objid = pg_proc.oid AND dep.deptype = 'e'
    WHERE
      ns.nspname = 'public'
      AND dep.objid IS NULL
      AND pg_proc.prokind = '{prokind}'
    ",
  ))?;
  db.simple_entities(buffer_cmd.get(before..).unwrap_or_default(), buffer_idents).await?;
  buffer_cmd.truncate(before);
  Ok(())
}

#[inline]
pub(crate) async fn _sequences<D>(db: &mut D, results: &mut Vec<Identifier>) -> crate::Result<()>
where
  D: Database,
{
  db.simple_entities(
    "SELECT
      sequence_name AS generic_column
    FROM
      information_schema.sequences
    WHERE
      sequence_schema = 'public'",
    results,
  )
  .await?;
  Ok(())
}

#[inline]
pub(crate) async fn _schemas<D>(db: &mut D, identifiers: &mut Vec<Identifier>) -> crate::Result<()>
where
  D: Database,
{
  db.simple_entities(
    "SELECT
    pc_ns.nspname AS generic_column
  FROM
    pg_catalog.pg_namespace pc_ns
  WHERE
    nspname NOT IN ('information_schema', 'pg_catalog', 'public')
    AND nspname NOT LIKE 'pg_toast%'
    AND nspname NOT LIKE 'pg_temp_%'
  ",
    identifiers,
  )
  .await?;
  Ok(())
}

pub(crate) async fn _table_names<D>(
  buffer_cmd: &mut String,
  db: &mut D,
  results: &mut Vec<Identifier>,
  schema: &str,
) -> crate::Result<()>
where
  D: Database,
{
  let before = buffer_cmd.len();
  buffer_cmd.write_fmt(format_args!(
    "SELECT
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
      )",
  ))?;
  db.simple_entities(buffer_cmd.get(before..).unwrap_or_default(), results).await?;
  buffer_cmd.truncate(before);
  Ok(())
}

#[inline]
pub(crate) async fn _types<D>(db: &mut D, results: &mut Vec<Identifier>) -> crate::Result<()>
where
  D: Database,
{
  db.simple_entities(
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
        select oid from pg_catalog.pg_namespace where nspname = 'public'
      )
      AND dep.objid is null
      AND t.typtype != 'd'",
    results,
  )
  .await?;
  Ok(())
}

#[inline]
pub(crate) async fn _views<D>(db: &mut D, results: &mut Vec<Identifier>) -> crate::Result<()>
where
  D: Database,
{
  db.simple_entities(
    "
    SELECT
      relname AS generic_column
    FROM pg_catalog.pg_class c
      JOIN pg_namespace n ON n.oid = c.relnamespace
      LEFT JOIN pg_depend dep ON dep.objid = c.oid AND dep.deptype = 'e'
    WHERE c.relkind = 'v'
      AND  n.nspname = 'public'
      AND dep.objid IS NULL
    ",
    results,
  )
  .await?;
  Ok(())
}

fn _push_drop(
  (buffer_cmd, buffer_idents): (&mut String, &mut Vec<Identifier>),
  structure: &str,
) -> crate::Result<()> {
  for identifier in &*buffer_idents {
    buffer_cmd.write_fmt(format_args!("DROP {structure} \"{identifier}\" CASCADE;"))?;
  }
  buffer_idents.clear();
  Ok(())
}
