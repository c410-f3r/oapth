//! Parse migration

#![allow(missing_docs)]
#![no_main]

use libfuzzer_sys::fuzz_target;
use oapth::parse_migration;

fuzz_target!(|data: &[u8]| {
    let _ = parse_migration(data);
});
