//! Validate

#![allow(missing_docs)]
#![no_main]

use libfuzzer_sys::fuzz_target;
use oapth::{Commands, Migration, MigrationGroup};
use tokio::runtime::Runtime;

#[derive(Debug, arbitrary::Arbitrary)]
struct Data {
  m_name: String,
  m_sql_down: String,
  m_sql_up: String,
  m_version: i32,
  mg_name: String,
  mg_version: i32,
}

fuzz_target!(|data: Data| {
  let mut rt = Runtime::new().unwrap();
  rt.block_on(async {
    let mut c = Commands::new(());
    let mg = MigrationGroup::new(data.mg_version, data.mg_name);
    let ms = [Migration::new(
      [].iter().copied(),
      None,
      data.m_version,
      data.m_name,
      data.m_sql_down,
      data.m_sql_up,
    )];
    let _ = c.validate(&mg, ms.iter());
  });
});
