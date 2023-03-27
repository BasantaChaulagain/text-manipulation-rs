//use reqwest;
//use std::collections::HashMap;
use curl::easy::{Easy, List};
use serde_json::{Value};
use std::io::Read;
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct Foo<'a> {
    text: &'a str, 
    target_lang: &'a str, 
}

fn read_secret(path: &str) -> String {
    fs::read_to_string(path)
        .expect("Failed to parse file.")
}

struct GetRequest<'a> {
    endpoint: &'a str, 
    auth: &'a String, 
    headers: Option<Vec<&'a str>>
}

impl<'a> GetRequest<'a> {
    fn execute(&self) -> Result<Value, Box<dyn std::error::Error>> {
        let mut easy = Easy::new();
        easy.url(&self.endpoint).unwrap();

        let mut list = List::new();
        let header = format!("Authorization: {}", &self.auth);
        list.append(header.as_str()).unwrap();

        if let Some(h) = &self.headers {
            for head in h.into_iter() {
                list.append(head).unwrap();
            }
        }

        // for l in &list {
        //     println!("L: {:#?}", l);
        // }
        println!("{:?}", list);

        easy.http_headers(list).unwrap();

        let mut data = Vec::new();
        {
            let mut transfer = easy.transfer();
            transfer.write_function(|d| {
                data.extend_from_slice(d);
        
                Ok(d.len())
            }).unwrap();
            transfer.perform().unwrap();
        }

        let s = match std::str::from_utf8(&data) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        println!("{}", s);

        let v : Value = serde_json::from_str(s)?;

        // if let Some(x) = &self.headers {
        //     Ok(s)
        // } else {
        //     Ok(v)
        // }

        Ok(v)
    }
}

struct PostRequest<'a> {
    endpoint: &'a str, 
    auth: &'a String, 
    params: Vec<&'a str>
}

impl<'a> PostRequest<'a> {
    fn execute(&self) -> Result<Value, Box<dyn std::error::Error>> {
        let mut easy = Easy::new();
        easy.url(&self.endpoint).unwrap();

        let mut list = List::new();
        let header = format!("Authorization: {}", &self.auth);
        list.append(header.as_str()).unwrap();
        easy.http_headers(list).unwrap();

        let body = &self.params.join("&");
        let mut bytes = body.as_bytes();
        easy.post(true).unwrap();
        easy.post_field_size(bytes.len() as u64).unwrap();

        let mut out = Vec::new();
        {
            let mut transfer = easy.transfer();
            transfer.read_function(|buf| {
                Ok(bytes.read(buf).unwrap_or(0))
            }).unwrap();

            transfer.write_function(|d| {
                out.extend_from_slice(d);
                Ok(d.len())
            }).unwrap();

            transfer.perform().unwrap();
        }

        let s = match std::str::from_utf8(&out) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        // println!("{}", s);

        let v : Value = serde_json::from_str(s)?;

        Ok(v)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let secret = read_secret("secret.txt");
    let key : String = format!("DeepL-Auth-Key {}", secret);

    let request = "https://api-free.deepl.com/v2/usage";
    let get_req = GetRequest {
        endpoint: request, 
        auth: &key, 
        headers: None
    };
    let resp = get_req.execute();
    if let Ok(v) = resp {
        println!("character_count: {}", v["character_count"]);
        println!("character_limit: {}", v["character_limit"]);
    }

    // text translation
    // let req2 = "https://api-free.deepl.com/v2/translate";
    // let mut p : Vec<&str> = Vec::new();
    // p.push("text=There once was a man from Peru.");
    // p.push("target_lang=DE");

    // let post = PostRequest {
    //     endpoint: req2, 
    //     auth: &key, 
    //     params: p
    // };
    // let resp = post.execute();

    // if let Ok(v) = resp {
    //     let q = &v["translations"][0];
    //     println!("detected_source_language: {}", q["detected_source_language"]);
    //     println!("text: {}", q["text"]);
    // }

    //see if we can make a glossary
    // let gloss = "https://api-free.deepl.com/v2/glossaries";

    // let mut glossary : Vec<&str> = Vec::new();
    // glossary.push("Hello\tGuten Tag");
    // glossary.push("Bye\tTschüss");
    // glossary.push("Foo\tBar");

    // let glossary_str = glossary.join("\n");

    // let mut args : Vec<&str> = Vec::new();
    // args.push("name=My Glossary");
    // args.push("source_lang=en");
    // args.push("target_lang=de");
    // let temp = format!("entries={}", glossary_str);
    // args.push(temp.as_str());
    // args.push("entries_format=tsv");

    // let create_glossary = PostRequest {
    //     endpoint: gloss, 
    //     auth: &key, 
    //     params: args
    // };

    // let resp = create_glossary.execute();
    // if let Ok(v) = resp {
    //     println!("glossary_id: {}", v["glossary_id"]);
    //     println!("ready: {}", v["ready"]);
    //     println!("name: {}", v["name"]);
    //     println!("source_lang: {}", v["source_lang"]);
    //     println!("target_lang: {}", v["target_lang"]);
    //     println!("creation_time: {}", v["creation_time"]);
    //     println!("entry_count: {}", v["entry_count"]);
    // } else {
    //     println!("Failed to create glossary.");
    // }

    // retrieve glossary
    let glossary_id = "91b13c25-15bf-4481-9e3e-51b23a60e07f";
    let gloss_req = format!("https://api-free.deepl.com/v2/glossaries/{}/entries", glossary_id);

    let mut accept: Vec<&str> = Vec::new();
    accept.push("Accept: text/tab-separated-values");

    let get_glossary = GetRequest {
        endpoint: &gloss_req, 
        auth: &key, 
        headers: None
    };

    let resp = get_glossary.execute();
    if let Ok(v) = resp {
        println!("RESP BODY: \n{}", v);
    } else {
        //this happens, because JSON is not returned
        println!("UH OH");
    }

    //Use dictionary in translation
    let post2 = "https://api-free.deepl.com/v2/translate";
    let mut p : Vec<&str> = Vec::new();
    p.push("text=Foo is cool. Hello, World! Bye!");
    p.push("source_lang=EN");
    p.push("target_lang=DE");
    p.push("glossary_id=91b13c25-15bf-4481-9e3e-51b23a60e07f");

    let post = PostRequest {
        endpoint: post2, 
        auth: &key, 
        params: p
    };
    let resp = post.execute();

    if let Ok(v) = resp {
        let q = &v["translations"][0];
        println!("detected_source_language: {}", q["detected_source_language"]);
        println!("text: {}", q["text"]);
    }

    Ok(())
}
