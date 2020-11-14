//! Parse migration

#![allow(missing_docs)]
#![no_main]

use libfuzzer_sys::fuzz_target;
use oapth::{parse_cfg, parse_migration};
use std::path::Path;

fuzz_target!(|data: &[u8]| {
    let _ = parse_cfg(data, &Path::new("."));
    let _ = parse_migration(data);
});
