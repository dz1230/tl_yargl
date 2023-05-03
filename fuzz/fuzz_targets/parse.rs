#![no_main]
use libfuzzer_sys::fuzz_target;
extern crate tl_yargl;

fuzz_target!(|data: &str| {
    let _ = tl_yargl::parse(data, tl_yargl::ParserOptions::default());
});
