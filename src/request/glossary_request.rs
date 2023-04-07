use std::collections::HashMap;

use crate::deepl::{SourceLang, TargetLang, Glossary, DeepLKey};
use crate::request::http_request::{HttpRequest, RequestType};
use serde_json::Value;

pub fn create_glossary_from_string(auth: &DeepLKey, name: String, source_lang: SourceLang, target_lang: TargetLang, entries: String) -> Result<Value, Box<dyn std::error::Error>> {
    let endpoint = "https://api-free.deepl.com/v2/glossaries";
    let mut params : Vec<String> = Vec::new();

    params.push(format!("name={}", name));
    params.push(format!("source_lang={}", source_lang));
    params.push(format!("target_lang={}", target_lang));
    params.push(format!("entries={}", entries));
    params.push("entries_format=tsv".to_string());

    let request = HttpRequest {
        endpoint: endpoint, 
        auth: &auth.key, 
        headers: None, 
        body: Some(params), 
        request_type: RequestType::Post, 
    };

    let res = request.execute();

    if let Ok(v) = res {
        Ok(v)
    } else {
        panic!("I forgot how to pass error statements")
    }
}

pub fn get_glossaries(auth: &DeepLKey) -> Result<Vec<Glossary>, Box<dyn std::error::Error>> {
    let endpoint = "https://api-free.deepl.com/v2/glossaries";
    let mut glossaries : Vec<Glossary> = Vec::new();

    let request = HttpRequest {
        endpoint: endpoint,
        auth: &auth.key,
        headers: None, 
        body: None, 
        request_type: RequestType::Get
    };

    let res = request.execute();

    if let Ok(v) = res {
        let g = &v["glossaries"];
        let arr = g.as_array().unwrap();

        for entry in arr.to_owned() {
            let gloss = Glossary::new(entry);
            glossaries.push(gloss);
        }

        Ok(glossaries)
    } else {
        panic!("I forgot how to pass error statements")
    }
}

pub fn get_glossary(auth: &DeepLKey, glossary_id: String) -> Result<Glossary, Box<dyn std::error::Error>> {
    let endpoint = format!("https://api-free.deepl.com/v2/glossaries/{}", glossary_id);

    let request = HttpRequest {
        auth: &auth.key, 
        endpoint: &endpoint.as_str(), 
        headers: None, 
        body: None, 
        request_type: RequestType::Get
    };

    let res = request.execute();

    if let Ok(g) = res {
        let glossary = Glossary::new(g);

        Ok(glossary)
    } else {
        panic!("Git gud");
    }
}

pub fn delete_glossary(auth: &DeepLKey, glossary_id: String) -> Result<(), Box<dyn std::error::Error>> {
    let endpoint = format!("https://api-free.deepl.com/v2/glossaries/{}", glossary_id);

    let request = HttpRequest {
        endpoint: endpoint.as_str(), 
        auth: &auth.key, 
        headers: None, 
        body: None, 
        request_type: RequestType::Delete
    };

    let res = request.execute();

    if let Ok(m) = res {
        println!("MESSAGE: {}", m);

        //TODO: return error if message "Not found" returned

        Ok(())
    } else {
        panic!("Git gud");
    }
}

pub fn get_glossary_entries(auth: &DeepLKey, glossary_id: String) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let endpoint = format!("https://api-free.deepl.com/v2/glossaries/{}/entries", glossary_id);

    // let params = vec![];
    let mut params : Vec<String> = Vec::new();
    params.push(String::from("Accept: text/tab-separated-values"));

    let request = HttpRequest {
        endpoint: endpoint.as_str(), 
        auth: &auth.key, 
        headers: Some(params), 
        body: None, 
        request_type: RequestType::Get
    };

    let res = request.execute();

    if let Ok(map) = res {
        let mut hm : HashMap<String, String> = HashMap::new();

        //TODO: FIX
        // let rows : Vec<&str> = map.split("\n").collect();
        // for row in rows {
        //     let key_val : Vec<&str> = row.split("\t").collect();

        //     hm.insert(key_val[0].to_string(), key_val[1].to_string());
        // }

        Ok(hm)
    } else {
        panic!("Git gud");
    }
}