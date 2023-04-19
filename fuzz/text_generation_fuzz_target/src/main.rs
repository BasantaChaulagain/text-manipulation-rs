#[macro_use]
extern crate afl;
extern crate text_manipulation_rs;

use std::convert::TryFrom;

fn main() {
    fuzz!(|data: &[u8]| {
        let index = data.len() as i32;
        text_manipulation_rs::text_manipulation::generate_text_for_language(index, false);
    });
}
