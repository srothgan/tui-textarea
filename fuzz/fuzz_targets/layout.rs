#![no_main]

libfuzzer_sys::fuzz_target!(|data: &[u8]| {
    tui_textarea_fuzz::run(tui_textarea_fuzz::Target::Layout, data);
});
