
#![no_main]
use libfuzzer_sys::fuzz_target;
use nexform_core::api::parser::Parser;

fuzz_target!(|data: &[u8]| {
    let mut parser = Parser::new();
    let _ = nexform_core::api::parser::Parser::new().decode_payload(data);
});
