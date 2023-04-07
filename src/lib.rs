pub mod deepl;
pub mod request;

use std::fs;

pub fn read_secret(path: &str) -> String {
    fs::read_to_string(path)
        .expect("Failed to parse file.")
}

pub struct DeepLKey {
    pub key: String, 
}

impl<'a> DeepLKey {
    pub fn new(path: &'a str) -> Result<DeepLKey, std::io::Error> {
        let key = fs::read_to_string(path)
            .expect("Failed to parse file.");

        Ok(DeepLKey {key})
    }
}