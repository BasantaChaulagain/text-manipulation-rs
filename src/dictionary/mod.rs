use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
struct Definition {
    #[serde(rename = "shortdef")]
    short_definitions: Vec<String>,
}

fn get_secret_key() -> Result<String, Box<dyn std::error::Error>> {
    let key = fs::read_to_string("dict_secret.txt")?;
    Ok(key)
}

pub fn get_meaning(word: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let key = get_secret_key()?;

    let url = format!(
        "https://dictionaryapi.com/api/v3/references/collegiate/json/{}/?key={}",
        word, key
    );

    let response = reqwest::blocking::get(&url)?.json::<Value>()?;

    let first_entry = response
        .as_array()
        .ok_or("Unexpected response format: expected JSON array; no entry found")?
        .get(0)
        .ok_or("No definitions found")?;

    let definitions = first_entry["shortdef"]
        .as_array()
        .ok_or("Unexpected response format: expected 'shortdef' array field")?
        .iter()
        .map(|def| def.as_str().unwrap_or("").to_owned())
        .collect();
    
    Ok(definitions)
}