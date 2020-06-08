#![no_main]
use libfuzzer_sys::fuzz_target;
extern crate fuzz_targets;

use fuzz_targets::fuzz_mmr_push_bytes;

fuzz_target!(|data: &[u8]| {
    let _ = fuzz_mmr_push_bytes(data);
});