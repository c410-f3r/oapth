//! Validate

#![allow(missing_docs)]
#![no_main]

use core::iter::once;
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
    let mut c = Commands::with_back_end(());
    let mg = MigrationGroup::new(data.mg_name, data.mg_version);
    let _ = c.validate(
      mg.m_g_ref(),
      once(
        Migration::from_parts(
          &[][..],
          None,
          data.m_version,
          data.m_name,
          data.m_sql_up,
          data.m_sql_down,
        )
        .unwrap()
        .m_ref(),
      ),
    );
  });
});
