use arrayvec::ArrayString;
use core::fmt::Write;

pub const _CREATE_MIGRATION_TABLES: &str = concat!(
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

// https://github.com/flyway/flyway/blob/master/flyway-core/src/main/java/org/flywaydb/core/internal/database/postgresql/PostgreSQLSchema.java
#[inline]
pub fn _all_tables(schema: &str) -> crate::Result<ArrayString<[u8; 1024]>> {
  let mut buffer = ArrayString::new();
  buffer.write_fmt(
    format_args!(
      "
        SELECT
          all_tables.table_name
        FROM
          information_schema.tables all_tables
          -- that don't depend on an extension
          LEFT JOIN pg_depend dep ON dep.objid = (quote_ident(all_tables.table_schema)||'.'||quote_ident(all_tables.table_name))::regclass::oid AND dep.deptype = 'e'
        WHERE
          -- in this schema
          table_schema='{schema}'
          -- that are real tables (as opposed to views)
          AND table_type='BASE TABLE'
          -- with no extension depending on them
          AND dep.objid IS NULL
          -- and are not child tables (= do not inherit from another table).
          AND NOT (
            SELECT EXISTS (SELECT inhrelid FROM pg_catalog.pg_inherits
            WHERE inhrelid = (quote_ident(all_tables.table_schema)||'.'||quote_ident(all_tables.table_name))::regclass::oid)
          )
      ",
      schema = schema
    )
  )?;
  Ok(buffer)
}

#[inline]
pub fn _clean() -> crate::Result<ArrayString<[u8; 2048]>> {
  let mut buffer = ArrayString::new();
  buffer.write_fmt(format_args!(
    "      
      -- Drop user schemas

      DO $$ DECLARE
        schema_rec RECORD;
      BEGIN
        FOR schema_rec IN (
          SELECT
            pc_ns.nspname AS name
          FROM
            pg_catalog.pg_namespace pc_ns
          WHERE
            nspname NOT IN ('information_schema', 'pg_catalog', 'public')
            AND nspname NOT LIKE 'pg_toast%'
            AND nspname NOT LIKE 'pg_temp_%'
        ) LOOP
          EXECUTE 'DROP SCHEMA IF EXISTS ' || quote_ident(schema_rec.name) || ' CASCADE';
        END LOOP;
      END $$;

      DO $$ DECLARE
        domain_rec RECORD;
        table_rec RECORD;
      BEGIN
        -- Drop domains

        FOR domain_rec IN (
          SELECT
            t.typname as name
          FROM pg_catalog.pg_type t
            LEFT JOIN pg_catalog.pg_namespace n ON n.oid = t.typnamespace
            LEFT JOIN pg_depend dep ON dep.objid = t.oid AND dep.deptype = 'e'
          WHERE t.typtype = 'd'
            AND n.nspname = 'public'
            AND dep.objid IS NULL
        ) LOOP
          EXECUTE 'DROP DOMAIN ' || quote_ident(domain_rec) || '';
        END LOOP;

        -- Drop tables

        FOR table_rec IN (SELECT tablename FROM pg_tables WHERE schemaname = 'public') LOOP
          EXECUTE 'DROP TABLE IF EXISTS ' || quote_ident(table_rec.tablename) || ' CASCADE';
        END LOOP;
      END $$;
      ",
  ))?;
  Ok(buffer)
}
