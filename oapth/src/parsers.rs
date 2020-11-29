pub mod parse_migration;

use crate::group_dir_name_parts;
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
    if group.is_empty() || !path.is_dir() || group_dir_name_parts(name).is_none() {
      continue;
    }
    groups
      .try_push(path)
      .map_err(|_| crate::Error::Other("There can't be more than 16 groups in a configuration"))?;
  }
  Ok(groups)
}

#[cfg(test)]
mod tests {
  use crate::parse_cfg;
  use std::path::Path;

  #[test]
  fn parse_cfg_works() {
    let cfg = br#"
      ../oapth-test-utils/migrations/1__initial
      ../oapth-test-utils/migrations/2__more_stuff
    "#;
    let groups = parse_cfg(&cfg[..], Path::new("../oapth-test-utils")).unwrap();
    assert_eq!(groups.len(), 2);
  }
}
