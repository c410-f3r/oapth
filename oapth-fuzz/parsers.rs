//! Parse migration

#![allow(missing_docs)]
#![no_main]

use libfuzzer_sys::fuzz_target;
use oapth::Config;
use oapth_commons::{parse_root_cfg_raw, parse_unified_migration};
use std::path::Path;

fuzz_target!(|data: &[u8]| {
  let _ = parse_root_cfg_raw(data, Path::new("."));
  let _ = parse_unified_migration(data);

  let s = if let Ok(rslt) = core::str::from_utf8(data) {
    rslt
  } else {
    return;
  };

  let c = Config::with_url(s);
  let _ = c.database();
  let _ = c.full_host();
  let _ = c.host();
  let _ = c.name();
  let _ = c.password();
  let _ = c.port();
  let _ = c.url();
  let _ = c.user();
});
