#[macro_use]
extern crate afl;
extern crate text_manipulation_rs;

use text_manipulation_rs::my_memory::translate_q_langpair;

fn main() {
    fuzz!(|data: &[u8]| {
        let len  = data.len();
        let size = 5;

        let q = &data[..len-size];
        let s = match std::str::from_utf8(q) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        let langpair = &data[len-size..];
        let s2 = match std::str::from_utf8(langpair) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        let _ = translate_q_langpair(s.to_string(), s2.to_string());
    });
}
