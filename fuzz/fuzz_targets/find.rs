#![no_main]
use libfuzzer_sys::fuzz_target;
extern crate tl_yargl;

fuzz_target!(|data: (&[u8], u8)| {
    let (haystack, needle) = data;
    tl_yargl::simd::find(haystack, needle);
});
