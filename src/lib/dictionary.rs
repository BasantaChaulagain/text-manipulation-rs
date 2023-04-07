use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
struct Definition {
    #[serde(rename = "shortdef")]
    short_definitions: Vec<String>,
}

pub fn get_meaning(word: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let key = "4129012f-10c6-4b89-b6d4-e0fe7e60154f";

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