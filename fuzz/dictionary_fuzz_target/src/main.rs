#[macro_use]
extern crate afl;
extern crate text_manipulation_rs;

fn main() {
    fuzz!(|data: &[u8]|{
        // let word = std::str::from_utf8(data);
        if let Ok(word) = std::str::from_utf8(data){
            let _ = text_manipulation_rs::dictionary::get_meaning(word);
        }
    });
}
