use crate::Dbs;
use std::io::{BufRead, BufReader, Read};

#[derive(Debug, Default)]
pub struct ParsedMigration {
  pub dbs: Dbs,
  pub sql_down: String,
  pub sql_up: String,
}

/// Gets all information related to a migration from a reading source.
#[inline]
pub fn parse_migration<R>(read: R) -> crate::Result<ParsedMigration>
where
  R: Read,
{
  let mut br = BufReader::new(read);
  let mut overall_buffer = String::with_capacity(16);
  let mut parsed_migration = ParsedMigration::default();

  iterations(&mut overall_buffer, &mut br, |_| false)?;

  if let Some(rslt) = overall_buffer.split("-- oapth dbs").nth(1) {
    for db_str in rslt.split(',') {
      if let Ok(db) = db_str.trim().parse() {
        parsed_migration.dbs.push(db);
      }
    }
    iterations(&mut overall_buffer, &mut br, |_| false)?;
  }

  if overall_buffer.find("-- oapth UP").is_none() {
    return Err(crate::Error::IncompleteSqlFile);
  }

  iterations(&mut overall_buffer, &mut br, |str_read| {
    str_read.find("-- oapth DOWN").is_none()
  })?;
  if let Some(rslt) = overall_buffer.rsplit("-- oapth DOWN").nth(1) {
    parsed_migration.sql_up = rslt.trim().into();
  }

  iterations(&mut overall_buffer, &mut br, |_| true)?;
  parsed_migration.sql_down = overall_buffer.trim().into();

  if parsed_migration.sql_down.is_empty() || parsed_migration.sql_up.is_empty() {
    return Err(crate::Error::IncompleteSqlFile);
  }
  Ok(parsed_migration)
}

#[inline]
pub fn iterations<F, R>(
  overall_buffer: &mut String,
  br: &mut BufReader<R>,
  mut cb: F,
) -> crate::Result<()>
where
  F: FnMut(&str) -> bool,
  R: Read,
{
  overall_buffer.clear();
  let mut bytes_read = 0;

  loop {
    let curr_bytes_read = br.read_line(overall_buffer)?;

    if curr_bytes_read == 0 {
      break;
    }

    let str_read = if let Some(rslt) = overall_buffer.get(bytes_read..) { rslt } else { break };

    if str_read.trim_end().is_empty() {
      continue;
    }

    if !cb(str_read) {
      break;
    }

    bytes_read = bytes_read.saturating_add(curr_bytes_read);
  }

  Ok(())
}

#[cfg(test)]
mod tests {
  use crate::{parse_migration, Database};

  #[test]
  fn must_have_obrigatory_params() {
    assert!(parse_migration(&[][..]).is_err());
  }

  #[test]
  fn parses_optional_dbs() {
    let s = "-- oapth UP\nSOMETHING\n-- oapth DOWN\nSOMETHING";
    let no_dbs = parse_migration(s.as_bytes()).unwrap();
    assert!(no_dbs.dbs.is_empty());

    let s = "-- oapth dbs\n-- oapth UP\nSOMETHING\n-- oapth DOWN\nSOMETHING";
    let no_dbs_with_declaration = parse_migration(s.as_bytes()).unwrap();
    assert!(no_dbs_with_declaration.dbs.is_empty());

    let s = "-- oapth dbs mssql,pg\n-- oapth UP\nSOMETHING\n-- oapth DOWN\nSOMETHING";
    let two_dbs = parse_migration(s.as_bytes()).unwrap();
    assert_eq!(two_dbs.dbs[0], Database::Mssql);
    assert_eq!(two_dbs.dbs[1], Database::Pg);

    let s = "-- oapth dbs bird,apple\n-- oapth UP\nSOMETHING\n-- oapth DOWN\nSOMETHING";
    let unknown_dbs = parse_migration(s.as_bytes()).unwrap();
    assert!(unknown_dbs.dbs.is_empty());
  }

  #[test]
  fn parses_mandatory_params() {
    let s = "\n-- oapth UP\n\nSOMETHING\nFOO\n\n-- oapth DOWN\n\nBAR\n";
    let rslt = parse_migration(s.as_bytes()).unwrap();
    assert_eq!("SOMETHING\nFOO", rslt.sql_up);
    assert_eq!("BAR", rslt.sql_down);
  }
}
