use crate::_group_dir_name_parts;
use arrayvec::ArrayVec;
use std::{
  io::{BufRead, BufReader, Read},
  path::{Path, PathBuf},
};

/// All paths to directories that contain migrations
#[inline]
pub fn parse_cfg<R>(read: R, root: &Path) -> crate::Result<ArrayVec<[PathBuf; 16]>>
where
  R: Read,
{
  let mut groups = ArrayVec::new();
  let mut br = BufReader::new(read);
  loop {
    let mut group = String::new();
    if br.read_line(&mut group)? == 0 {
      break;
    }
    let path = root.join(group.trim());
    let name_opt = || path.file_name()?.to_str();
    let name = if let Some(rslt) = name_opt() {
      rslt
    } else {
      continue;
    };
    if group.is_empty() || !path.is_dir() || _group_dir_name_parts(name).is_none() {
      continue;
    }
    groups
      .try_push(path)
      .map_err(|_| crate::Error::Other("There can't be more than 16 groups in a configuration"))?;
  }
  Ok(groups)
}

/// Gets all information related to a migration from a reading source.
#[inline]
pub fn parse_migration<R>(read: R) -> crate::Result<(String, String)>
where
  R: Read,
{
  let mut br = BufReader::new(read);

  let mut overall_buffer = String::with_capacity(16);

  br.read_line(&mut overall_buffer)?;
  if overall_buffer.trim() != "-- oapth UP" {
    return Err(crate::Error::IncompleteSqlFile);
  }

  overall_buffer.clear();
  let mut sql_up = String::new();
  let mut total_bytes: usize = 0;
  loop {
    let bytes_read = br.read_line(&mut overall_buffer)?;
    let read_str = if let Some(rslt) = overall_buffer.get(total_bytes..) { rslt } else { break };
    let oapth_down = "-- oapth DOWN";
    if let Some(idx) = read_str.rfind(oapth_down) {
      let sql_up_len = total_bytes.saturating_add(idx);
      let split_off_idx = sql_up_len.saturating_add(oapth_down.len());
      let after_oapth_down = overall_buffer.split_off(split_off_idx);
      sql_up = overall_buffer;
      sql_up.truncate(sql_up_len);
      overall_buffer = after_oapth_down;
      break;
    }
    total_bytes = total_bytes.saturating_add(bytes_read);
  }

  if sql_up.is_empty() {
    return Err(crate::Error::IncompleteSqlFile);
  }

  loop {
    if br.read_line(&mut overall_buffer)? == 0 {
      break;
    }
  }

  let sql_down = overall_buffer;
  Ok((sql_up, sql_down))
}

#[cfg(test)]
mod tests {
  use crate::{parse_cfg, parse_migration};
  use std::{fs::File, path::Path};

  #[test]
  fn parse_cfg_works() {
    let cfg = br#"
      ../oapth-test-utils/migrations/1__initial
      ../oapth-test-utils/migrations/2__more_stuff
    "#;
    let groups = parse_cfg(&cfg[..], Path::new("../oapth-test-utils")).unwrap();
    assert_eq!(groups.len(), 2);
  }

  #[test]
  fn parse_migration_works() {
    let path = Path::new("../oapth-test-utils/migrations/1__initial/1__create_author.sql");
    let file = File::open(path).unwrap();
    let (up, down) = parse_migration(file).unwrap();
    assert_eq!(
      "
CREATE TABLE author (
  id INT NOT NULL PRIMARY KEY,
  first_name VARCHAR(50) NOT NULL,
  last_name VARCHAR(50) NOT NULL,
  email VARCHAR(100) NOT NULL
);

",
      up
    );
    assert_eq!(
      "

DROP TABLE author;
",
      down
    );
  }
}
