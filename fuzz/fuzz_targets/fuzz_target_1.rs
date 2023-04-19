#![no_main]

#[path = "src/lib/my_memory.rs"]
use text_manipulation_rs::text_manipulation::translate_q_langpair;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: (String, String)| {
    let q = &data.0;
    let langpair = &data.1;
    let _ = translate_q_langpair(q.to_string(), langpair.to_string());
});
