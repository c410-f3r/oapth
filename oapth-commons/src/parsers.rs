pub(crate) mod parse_migration;

use crate::dir_name_parts;
use arrayvec::ArrayVec;
use std::{
  fs::File,
  io::{BufRead, BufReader, Read},
  path::{Path, PathBuf},
};

/// All paths to directories that contain migrations
#[inline]
pub fn parse_root_cfg(cfg_path: &Path) -> crate::Result<ArrayVec<[PathBuf; 16]>> {
  let cfg_dir = cfg_path.parent().unwrap_or_else(|| Path::new("."));
  parse_root_cfg_raw(File::open(cfg_path)?, cfg_dir)
}

/// Similar to `parse_root_cfg`, takes a stream of bytes and a base path as arguments.
#[inline]
pub fn parse_root_cfg_raw<R>(read: R, root: &Path) -> crate::Result<ArrayVec<[PathBuf; 16]>>
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
    if group.is_empty() || !path.is_dir() || dir_name_parts(name).is_err() {
      continue;
    }
    groups.try_push(path).map_err(|_| crate::Error::MaxNumGroups)?;
  }
  Ok(groups)
}

#[cfg(test)]
mod tests {
  use crate::parsers::parse_root_cfg_raw;
  use std::path::Path;

  #[test]
  fn parse_root_cfg_works() {
    let cfg = br#"
      ../oapth-test-utils/migrations/1__initial
      ../oapth-test-utils/migrations/2__more_stuff
    "#;
    let groups = parse_root_cfg_raw(&cfg[..], Path::new("../oapth-test-utils")).unwrap();
    assert_eq!(groups.len(), 2);
  }
}
