#![no_main]
#[macro_use]
extern crate libfuzzer_sys;

fuzz_target!(|data: &[u8]| {
  let compressed_sample = archivelib_sys::do_compress(&data);
});
