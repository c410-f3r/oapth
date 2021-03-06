use crate::{Database, Repeatability};
use std::io::{BufRead, BufReader, Read};

#[derive(Debug, Default)]
pub struct ParsedMigration {
  pub cfg: MigrationCfg,
  pub sql_down: String,
  pub sql_up: String,
}

#[derive(Debug, Default)]
pub struct MigrationCfg {
  pub dbs: Vec<Database>,
  pub repeatability: Option<Repeatability>,
}

/// Gets all information related to a migration from a reading source.
#[inline]
pub fn parse_migration_cfg<R>(read: R) -> crate::Result<MigrationCfg>
where
  R: Read,
{
  let mut br = BufReader::new(read);
  let mut overall_buffer = String::with_capacity(16);
  let mut cfg = MigrationCfg::default();

  iterations(&mut overall_buffer, &mut br, |_| false)?;

  parse_dbs(&mut br, &mut cfg.dbs, &mut overall_buffer)?;

  parse_repeatability(&mut br, &mut overall_buffer, &mut cfg.repeatability)?;

  Ok(cfg)
}

/// Gets all information related to a migration from a reading source.
#[inline]
pub fn parse_unified_migration<R>(read: R) -> crate::Result<ParsedMigration>
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
        parsed_migration.cfg.dbs.push(db);
      }
    }
    iterations(&mut overall_buffer, &mut br, |_| false)?;
  }

  if let Some(rslt) = overall_buffer.split("-- oapth repeatability").nth(1) {
    if let Ok(repeatability) = rslt.trim().parse() {
      parsed_migration.cfg.repeatability = Some(repeatability);
    }
    iterations(&mut overall_buffer, &mut br, |_| false)?;
  }

  if overall_buffer.find("-- oapth UP").is_none() {
    return Err(crate::Error::IncompleteSqlFile);
  }

  iterations(&mut overall_buffer, &mut br, |str_read| str_read.find("-- oapth DOWN").is_none())?;

  if let Some(rslt) = overall_buffer.rsplit("-- oapth DOWN").nth(1) {
    parsed_migration.sql_up = rslt.trim().into();
  } else {
    parsed_migration.sql_up = overall_buffer.trim().into();
    return Ok(parsed_migration);
  }

  iterations(&mut overall_buffer, &mut br, |_| true)?;

  parsed_migration.sql_down = overall_buffer.trim().into();

  if parsed_migration.sql_up.is_empty() {
    return Err(crate::Error::IncompleteSqlFile);
  }

  Ok(parsed_migration)
}

#[inline]
fn iterations<F, R>(
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
    let trimmed = str_read.trim();

    bytes_read = bytes_read.saturating_add(curr_bytes_read);

    if trimmed.is_empty() || trimmed.starts_with("//") {
      continue;
    }

    if !cb(trimmed) {
      break;
    }
  }

  Ok(())
}

fn parse_dbs<R>(
  br: &mut BufReader<R>,
  dbs: &mut Vec<Database>,
  overall_buffer: &mut String,
) -> crate::Result<()>
where
  R: Read,
{
  if let Some(rslt) = overall_buffer.split("-- oapth dbs").nth(1) {
    for db_str in rslt.split(',') {
      if let Ok(db) = db_str.trim().parse() {
        dbs.push(db);
      }
    }
    iterations(overall_buffer, br, |_| false)?;
  }
  Ok(())
}

fn parse_repeatability<R>(
  br: &mut BufReader<R>,
  overall_buffer: &mut String,
  repeatability: &mut Option<Repeatability>,
) -> crate::Result<()>
where
  R: Read,
{
  if let Some(repeatability_str) = overall_buffer.split("-- oapth repeatability").nth(1) {
    if let Ok(parsed_repeatability) = repeatability_str.trim().parse::<Repeatability>() {
      *repeatability = Some(parsed_repeatability)
    }
    iterations(overall_buffer, br, |_| false)?;
  }
  Ok(())
}

#[cfg(test)]
mod tests {
  use crate::{parse_unified_migration, Database, Repeatability};

  #[test]
  fn does_not_take_into_consideration_white_spaces_and_comments() {
    let s = "// FOO\n\t\n-- oapth UP\nSOMETHING\nFOO\n";
    let rslt = parse_unified_migration(s.as_bytes()).unwrap();
    assert_eq!("SOMETHING\nFOO", rslt.sql_up);
  }

  #[test]
  fn must_have_obrigatory_params() {
    assert!(parse_unified_migration(&[][..]).is_err());
  }

  #[test]
  fn parses_optional_dbs() {
    let s = "-- oapth UP\nSOMETHING";
    let no_declaration = parse_unified_migration(s.as_bytes()).unwrap();
    assert!(no_declaration.cfg.dbs.is_empty());

    let s = "-- oapth dbs\n-- oapth UP\nSOMETHING";
    let with_initial_declaration = parse_unified_migration(s.as_bytes()).unwrap();
    assert!(with_initial_declaration.cfg.dbs.is_empty());

    let s = "-- oapth dbs bird,apple\n-- oapth UP\nSOMETHING";
    let with_incorrect_declaration = parse_unified_migration(s.as_bytes()).unwrap();
    assert!(with_incorrect_declaration.cfg.dbs.is_empty());

    let s = "-- oapth dbs mssql,pg\n-- oapth UP\nSOMETHING";
    let two_dbs = parse_unified_migration(s.as_bytes()).unwrap();
    assert_eq!(two_dbs.cfg.dbs[0], Database::Mssql);
    assert_eq!(two_dbs.cfg.dbs[1], Database::Pg);
  }

  #[test]
  fn parses_down() {
    let s = "\n-- oapth UP\n\nSOMETHING\nFOO\n\n-- oapth DOWN\n\nBAR\n";
    let rslt = parse_unified_migration(s.as_bytes()).unwrap();
    assert_eq!("SOMETHING\nFOO", rslt.sql_up);
    assert_eq!("BAR", rslt.sql_down);
  }

  #[test]
  fn parses_repeatability() {
    let s = "-- oapth UP\nSOMETHING";
    let no_declaration = parse_unified_migration(s.as_bytes()).unwrap();
    assert!(no_declaration.cfg.repeatability.is_none());

    let s = "-- oapth repeatability\n-- oapth UP\nSOMETHING";
    let with_initial_declaration = parse_unified_migration(s.as_bytes()).unwrap();
    assert!(with_initial_declaration.cfg.repeatability.is_none());

    let s = "-- oapth repeatability FOO\n-- oapth UP\nSOMETHING";
    let with_incorrect_declaration = parse_unified_migration(s.as_bytes()).unwrap();
    assert!(with_incorrect_declaration.cfg.repeatability.is_none());

    let s = "-- oapth repeatability always\n-- oapth UP\nSOMETHING";
    let always = parse_unified_migration(s.as_bytes()).unwrap();
    assert_eq!(always.cfg.repeatability, Some(Repeatability::Always));

    let s = "-- oapth repeatability on_checksum_change\n-- oapth UP\nSOMETHING";
    let on_checksum_change = parse_unified_migration(s.as_bytes()).unwrap();
    assert_eq!(on_checksum_change.cfg.repeatability, Some(Repeatability::OnChecksumChange));
  }

  #[test]
  fn parses_mandatory_params() {
    let s = "-- oapth UP\n\nSOMETHING\nFOO";
    let rslt = parse_unified_migration(s.as_bytes()).unwrap();
    assert_eq!("SOMETHING\nFOO", rslt.sql_up);
  }
}
