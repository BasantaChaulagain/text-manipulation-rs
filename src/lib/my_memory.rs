use reqwest;

struct TranslationRequest {
    q: String,
    langpair: String,
}

impl TranslationRequest {
    fn new(q: String, langpair: String) -> Self {
        Self { q, langpair }
    }
}

use reqwest::blocking::Client;

fn translate(request: TranslationRequest) -> Result<String, reqwest::Error> {
    let url = format!(
        "https://api.mymemory.translated.net/get?q={}&langpair={}",
        request.q, request.langpair
    );
    let client = Client::new();
    let response = client.get(&url).send()?;
    let response_text = response.text()?;
    Ok(response_text)
}

pub fn translate_q_langpair(q: String, langpair: String) -> String { 
    let translation_request = TranslationRequest::new(q, langpair);
    let response_text = translate(translation_request).unwrap();
    let response_json: serde_json::Value = serde_json::from_str(&response_text).unwrap();
    let translated_text = response_json["responseData"]["translatedText"]
        .as_str()
        .unwrap_or_default()
        .to_owned();
    translated_text
}
