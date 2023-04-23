#[macro_use]
extern crate afl;
extern crate text_manipulation_rs;

use text_manipulation_rs::request::translation_request::TranslationRequest;
use text_manipulation_rs::deepl::{DeepLKey, TargetLang};

fn main() {
    let path = "../../src/secret.txt";
    let auth = DeepLKey::new(path).unwrap();

    fuzz!(|data: &[u8]| {
        let text = match std::str::from_utf8(data) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        let tr = TranslationRequest::new(text, TargetLang::De);
        let request = tr.create_request(&auth);
        let res = request.execute();

        match res {
            Ok(_) => (), 
            Err(e) => panic!("{}", e)
        };
    });
}
