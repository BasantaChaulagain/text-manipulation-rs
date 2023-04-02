//use reqwest;
//use std::collections::HashMap;
use curl::easy::{Easy, List};
use serde_json::{Value};
use std::{io::Read, fmt::Display, str::FromStr, any::Any, collections::HashMap};
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

struct DeepLKey {
    key: String, 
}

impl<'a> DeepLKey {
    fn new(path: &'a str) -> Result<DeepLKey, std::io::Error> {
        let key = fs::read_to_string(path)
            .expect("Failed to parse file.");

        Ok(DeepLKey {
            key: key, 
        })
    }
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
        let header = format!("Authorization: DeepL-Auth-Key {}", &self.auth);
        list.append(header.as_str()).unwrap();

        if let Some(h) = &self.headers {
            for head in h.into_iter() {
                list.append(head).unwrap();
            }
        }

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

        let v : Value = serde_json::from_str(s)?;

        Ok(v)
    }

    //TODO: Refactor this, so execute method can return Value or text (based on Accept header?)
    //probably would use a trait for this
    fn execute_temp(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut easy = Easy::new();
        easy.url(&self.endpoint).unwrap();

        let mut list = List::new();
        let header = format!("Authorization: DeepL-Auth-Key {}", &self.auth);
        list.append(header.as_str()).unwrap();

        if let Some(h) = &self.headers {
            for head in h.into_iter() {
                list.append(head).unwrap();
            }
        }

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

        Ok(s.to_string())
    }
}

#[derive(Debug)]
struct PostRequest<'a> {
    endpoint: &'a str, 
    auth: &'a String, 
    params: Vec<String>
}

impl<'a> PostRequest<'a> {
    fn execute(&self) -> Result<Value, Box<dyn std::error::Error>> {
        let mut easy = Easy::new();
        easy.url(&self.endpoint).unwrap();

        let mut list = List::new();
        let header = format!("Authorization: DeepL-Auth-Key {}", &self.auth);
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

        // let foo = easy.response_code();
        // match foo {
        //     Ok(200) | Ok(201) => println!("LET'S GOOOOOOOOOOOOOO"), 
        //     Ok(f) => println!("HTML Status Code: {}", f), 
        //     Err(_) => println!("HOOOOOOOOOOOOOOOOOOOW?")
        // }

        let s = match std::str::from_utf8(&out) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        let v : Value = serde_json::from_str(s)?;

        Ok(v)
    }
}

struct DeleteRequest<'a> {
    endpoint: &'a str, 
    auth: &'a String, 
    headers: Option<Vec<&'a str>>
}

impl<'a> DeleteRequest<'a> {
    fn execute(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut easy = Easy::new();
        easy.url(&self.endpoint).unwrap();

        easy.custom_request("DELETE")?;

        let mut list = List::new();
        let header = format!("Authorization: DeepL-Auth-Key {}", &self.auth);
        list.append(header.as_str()).unwrap();

        if let Some(h) = &self.headers {
            for head in h.into_iter() {
                list.append(head).unwrap();
            }
        }

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

        if s.len() > 0 {
            let v : Value = serde_json::from_str(s)?;
            Ok(v["message"].as_str().unwrap().to_string())
        } else {
            Ok("".to_string())
        }
    }
}

struct TranslationRequest<'a> {
    text: Box<[&'a str]>, 
    source_lang: Option<SourceLang>, 
    target_lang: TargetLang, 
    split_sentences: Option<SplitSentences>, 
    preserve_formatting: Option<bool>, 
    formality: Option<Formality>, 
    glossary_id: Option<&'a str>, 
    tag_handling: Option<TagHandling>, 
    non_splitting_tags: Option<&'a str>, 
    outline_detection: Option<bool>, 
    splitting_tags: Option<&'a str>, 
    ignore_tags: Option<&'a str>, 
}

impl<'a> TranslationRequest<'a> {
    fn new(text: Box<[&'a str]>, target_lang: TargetLang) -> TranslationRequest {
        TranslationRequest {
            text: text, 
            source_lang: None, 
            target_lang: target_lang, 
            split_sentences: None, 
            preserve_formatting: Some(false), 
            formality: Some(Formality::Default), 
            glossary_id: None, 
            tag_handling: None, 
            non_splitting_tags: None, 
            outline_detection: None, 
            splitting_tags: None, 
            ignore_tags: None 
        }
    }

    fn create_request(&self, auth : &'a DeepLKey) -> PostRequest {
        let mut par : Vec<String> = Vec::new();

        let text_box = &*self.text;
        let text_format = format!("text={}", text_box[0]);
        par.push(text_format);

        if let Some(sl) = &self.source_lang {
            let source_lang_format = format!("source_lang={}", sl);
            par.push(source_lang_format);
        }

        let target_lang_format = format!("target_lang={}", &self.target_lang);
        par.push(target_lang_format);

        if let Some(split_sen) = &self.split_sentences {
            let split_sen_format = format!("split_sentences={}", split_sen);
            par.push(split_sen_format);
        }

        if let Some(pf) = &self.preserve_formatting {
            let pf_str = match pf {
                false => "0", 
                true => "1"
            };
            let pf_format = format!("preserve_formatting={}", pf_str);
            par.push(pf_format);
        }

        if let Some(form) = &self.formality {
            let form_format = format!("formality={}", form);
            par.push(form_format);
        }

        if let Some(gid) = &self.glossary_id {
            let gid_format = format!("glossary_id={}", gid);
            par.push(gid_format);
        }

        if let Some(th) = &self.tag_handling {
            let th_format = format!("tag_handling={}", th);
            par.push(th_format);
        }

        if let Some(nst) = &self.non_splitting_tags {
            let nst_format = format!("non_splitting_tags={}", nst);
            par.push(nst_format);
        }

        if let Some(od) = &self.outline_detection {
            let od_format = format!("outline_detection={}", od);
            par.push(od_format);
        }

        if let Some(st) = &self.splitting_tags {
            let st_format = format!("splitting_tags={}", st);
            par.push(st_format);
        }

        if let Some(it) = &self.ignore_tags {
            let it_format = format!("ignore_tags={}", it);
            par.push(it_format);
        }

        PostRequest {
            endpoint: "https://api-free.deepl.com/v2/translate", 
            auth: &auth.key, 
            params: par
        }
    }

    fn set_source_lang(mut self, sl: SourceLang) -> TranslationRequest<'a> {
        self.source_lang = Some(sl);
        self
    }

    fn set_split_sentences(mut self, split: SplitSentences) -> TranslationRequest<'a> {
        self.split_sentences = Some(split);
        self
    }

    fn set_preserve_formatting(mut self, pf: bool) -> TranslationRequest<'a> {
        self.preserve_formatting = Some(pf);
        self
    }

    fn set_formality(mut self, formality: Formality) -> TranslationRequest<'a> {
        self.formality = Some(formality);
        self
    }

    fn set_glossary_id(mut self, id: &'a str) -> TranslationRequest<'a> {
        self.glossary_id = Some(id);
        self
    }

    fn set_tag_handling(mut self, th: TagHandling) -> TranslationRequest<'a> {
        self.tag_handling = Some(th);
        self
    }

    fn set_non_splitting_tags(mut self, tags: &'a str) -> TranslationRequest<'a> {
        self.non_splitting_tags = Some(tags);
        self
    }

    fn set_outline_detection(mut self, od: bool) -> TranslationRequest<'a> {
        self.outline_detection = Some(od);
        self
    }

    fn set_splitting_tags(mut self, tags: &'a str) -> TranslationRequest<'a> {
        self.splitting_tags = Some(tags);
        self
    }

    fn set_ignore_tags(mut self, tags: &'a str) -> TranslationRequest<'a> {
        self.ignore_tags = Some(tags);
        self
    }
}

struct TranslationResponse {
    detected_source_language: String, 
    text: String, 
}

enum SourceLang {
    Bg, 
    Cs, 
    Da, 
    De, 
    El, 
    En, 
    Es, 
    Et, 
    Fi, 
    Fr, 
    Hu, 
    Id, 
    It, 
    Ja, 
    Ko, 
    Lt, 
    Lv, 
    Nb, 
    Nl, 
    Pl, 
    Pt, 
    Ro, 
    Ru, 
    Sk, 
    Sl, 
    Sv, 
    Tr, 
    Uk, 
    Zh,
}

impl Display for SourceLang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SourceLang::Bg => f.write_str("BG"),  
            SourceLang::Cs => f.write_str("CS"),  
            SourceLang::Da => f.write_str("DA"),  
            SourceLang::De => f.write_str("DE"),  
            SourceLang::El => f.write_str("EL"),  
            SourceLang::En => f.write_str("EN"),  
            SourceLang::Es => f.write_str("ES"),  
            SourceLang::Et => f.write_str("ET"),  
            SourceLang::Fi => f.write_str("FI"),  
            SourceLang::Fr => f.write_str("FR"),  
            SourceLang::Hu => f.write_str("HU"),  
            SourceLang::Id => f.write_str("ID"),  
            SourceLang::It => f.write_str("IT"),  
            SourceLang::Ja => f.write_str("JA"),  
            SourceLang::Ko => f.write_str("KO"),  
            SourceLang::Lt => f.write_str("LT"),  
            SourceLang::Lv => f.write_str("LV"),  
            SourceLang::Nb => f.write_str("NB"),  
            SourceLang::Nl => f.write_str("NL"),  
            SourceLang::Pl => f.write_str("PL"),  
            SourceLang::Pt => f.write_str("PT"),  
            SourceLang::Ro => f.write_str("RO"),  
            SourceLang::Ru => f.write_str("RU"),  
            SourceLang::Sk => f.write_str("SK"),  
            SourceLang::Sl => f.write_str("SL"),  
            SourceLang::Sv => f.write_str("SV"),  
            SourceLang::Tr => f.write_str("TR"),  
            SourceLang::Uk => f.write_str("UK"),  
            SourceLang::Zh => f.write_str("ZH"), 
        }
    }
}

impl FromStr for SourceLang {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BG" => Ok(SourceLang::Bg),
            "CS" => Ok(SourceLang::Cs),
            "DA" => Ok(SourceLang::Da),
            "DE" => Ok(SourceLang::De),
            "EL" => Ok(SourceLang::El),
            "EN" => Ok(SourceLang::En),
            "ES" => Ok(SourceLang::Es),
            "ET" => Ok(SourceLang::Et),
            "FI" => Ok(SourceLang::Fi),
            "FR" => Ok(SourceLang::Fr),
            "HU" => Ok(SourceLang::Hu),
            "ID" => Ok(SourceLang::Id),
            "IT" => Ok(SourceLang::It),
            "JA" => Ok(SourceLang::Ja),
            "KO" => Ok(SourceLang::Ko),
            "LT" => Ok(SourceLang::Lt),
            "LV" => Ok(SourceLang::Lv),
            "NB" => Ok(SourceLang::Nb),
            "NL" => Ok(SourceLang::Nl),
            "PL" => Ok(SourceLang::Pl),
            "PT" => Ok(SourceLang::Pt),
            "RO" => Ok(SourceLang::Ro),
            "RU" => Ok(SourceLang::Ru),
            "SK" => Ok(SourceLang::Sk),
            "SL" => Ok(SourceLang::Sl),
            "SV" => Ok(SourceLang::Sv),
            "TR" => Ok(SourceLang::Tr),
            "UK" => Ok(SourceLang::Uk),
            "ZH" => Ok(SourceLang::Zh), 
            _ => Err(())
        }
    }
}

// #[derive(Debug)]
enum TargetLang {
    Bg,
    Cs,
    Da,
    De,
    El,
    En,
    EnGb,
    EnUs,
    Es,
    Et,
    Fi,
    Fr,
    Hu,
    Id,
    It,
    Ja,
    Ko,
    Lt,
    Lv,
    Nb,
    Nl,
    Pl,
    Pt,
    PtBr,
    PtPt,
    Ro,
    Ru,
    Sk,
    Sl,
    Sv,
    Tr,
    Uk,
    Zh, 
}

impl Display for TargetLang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TargetLang::Bg => f.write_str("BG"), 
            TargetLang::Cs => f.write_str("CS"), 
            TargetLang::Da => f.write_str("DA"), 
            TargetLang::De => f.write_str("DE"), 
            TargetLang::El => f.write_str("EL"), 
            TargetLang::En => f.write_str("EN"), 
            TargetLang::EnGb => f.write_str("EN-GB"), 
            TargetLang::EnUs => f.write_str("EN-US"), 
            TargetLang::Es => f.write_str("ES"), 
            TargetLang::Et => f.write_str("ET"), 
            TargetLang::Fi => f.write_str("FI"), 
            TargetLang::Fr => f.write_str("FR"), 
            TargetLang::Hu => f.write_str("HU"), 
            TargetLang::Id => f.write_str("ID"), 
            TargetLang::It => f.write_str("IT"), 
            TargetLang::Ja => f.write_str("JA"), 
            TargetLang::Ko => f.write_str("KO"), 
            TargetLang::Lt => f.write_str("LT"), 
            TargetLang::Lv => f.write_str("LV"), 
            TargetLang::Nb => f.write_str("NB"), 
            TargetLang::Nl => f.write_str("NL"), 
            TargetLang::Pl => f.write_str("PL"), 
            TargetLang::Pt => f.write_str("PT"), 
            TargetLang::PtBr => f.write_str("PT-BR"), 
            TargetLang::PtPt => f.write_str("PT-PT"), 
            TargetLang::Ro => f.write_str("RO"), 
            TargetLang::Ru => f.write_str("RU"), 
            TargetLang::Sk => f.write_str("SK"), 
            TargetLang::Sl => f.write_str("SL"), 
            TargetLang::Sv => f.write_str("SV"), 
            TargetLang::Tr => f.write_str("TR"), 
            TargetLang::Uk => f.write_str("UK"), 
            TargetLang::Zh => f.write_str("ZH"),  
        }
    }
}

impl FromStr for TargetLang {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BG" => Ok(TargetLang::Bg),
            "CS" => Ok(TargetLang::Cs),
            "DA" => Ok(TargetLang::Da),
            "DE" => Ok(TargetLang::De),
            "EL" => Ok(TargetLang::El),
            "EN" => Ok(TargetLang::En),
            "EN-GB" => Ok(TargetLang::EnGb), 
            "EN-US" => Ok(TargetLang::EnUs), 
            "ES" => Ok(TargetLang::Es),
            "ET" => Ok(TargetLang::Et),
            "FI" => Ok(TargetLang::Fi),
            "FR" => Ok(TargetLang::Fr),
            "HU" => Ok(TargetLang::Hu),
            "ID" => Ok(TargetLang::Id),
            "IT" => Ok(TargetLang::It),
            "JA" => Ok(TargetLang::Ja),
            "KO" => Ok(TargetLang::Ko),
            "LT" => Ok(TargetLang::Lt),
            "LV" => Ok(TargetLang::Lv),
            "NB" => Ok(TargetLang::Nb),
            "NL" => Ok(TargetLang::Nl),
            "PL" => Ok(TargetLang::Pl),
            "PT" => Ok(TargetLang::Pt),
            "PT-BR" => Ok(TargetLang::PtBr),
            "PT-PT" => Ok(TargetLang::PtPt),
            "RO" => Ok(TargetLang::Ro),
            "RU" => Ok(TargetLang::Ru),
            "SK" => Ok(TargetLang::Sk),
            "SL" => Ok(TargetLang::Sl),
            "SV" => Ok(TargetLang::Sv),
            "TR" => Ok(TargetLang::Tr),
            "UK" => Ok(TargetLang::Uk),
            "ZH" => Ok(TargetLang::Zh), 
            _ => Err(())
        }
    }
}

enum SplitSentences {
    None, 
    PunctuationAndNewline, 
    Punctuation, 
}

impl Display for SplitSentences {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SplitSentences::None => f.write_str("0"), 
            SplitSentences::PunctuationAndNewline => f.write_str("1"), 
            SplitSentences::Punctuation => f.write_str("nonewlines"), 
        }
    }
}

// #[derive(Debug)]
enum Formality {
    Default, 
    More, 
    Less, 
    PreferMore, 
    PreferLess
}

impl Display for Formality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Formality::Default => f.write_str("default"), 
            Formality::More => f.write_str("more"), 
            Formality::Less => f.write_str("less"), 
            Formality::PreferMore => f.write_str("prefer_more"), 
            Formality::PreferLess => f.write_str("prefer_less"), 
        }
    }
}

enum TagHandling {
    Xml, 
    Html
}

impl Display for TagHandling {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TagHandling::Html => f.write_str("html"), 
            TagHandling::Xml => f.write_str("xml"), 
        }
    }
}

// #[derive(Debug)]
struct Glossary {
    glossary_id: String, 
    name: String, 
    ready: bool, 
    source_lang: SourceLang, 
    target_lang: TargetLang, 
    creation_time: String, 
    entry_count: u64
}

fn capitalize(s: &str) -> String {
    let mut vec : Vec<String> = Vec::new();

    //capitalize each character in given string
    for c in s.chars() {
        let upper = c.to_uppercase().collect::<String>();
        vec.push(upper);
    }

    vec.join("")
}

impl Glossary {
    fn new(v: Value) -> Glossary {
        let upper_source = capitalize(v["source_lang"].as_str().unwrap());
        let upper_target = capitalize(v["target_lang"].as_str().unwrap());

        Glossary {
            glossary_id: v["glossary_id"].to_string(),
            name: v["name"].to_string(),
            ready: v["ready"].as_bool().unwrap(),
            source_lang: SourceLang::from_str(&upper_source.as_str()).unwrap(),
            target_lang: TargetLang::from_str(&upper_target.as_str()).unwrap(),
            creation_time: v["creation_time"].to_string(),
            entry_count: v["entry_count"].as_u64().unwrap() 
        }
    }

    // fn create() -> Glossary {
    //     Glossary {
    //         glossary_id: (),
    //         name: (),
    //         ready: (),
    //         source_lang: (),
    //         target_lang: TargetL,
    //         creation_time: "".to_string(),
    //         entry_count: 0, 
    //     }
    // }
}

impl Display for Glossary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = format!("ID: {}\nName: {}\nReady: {}\nSourceLang: {}\nTargetLang: {}\n Creation time: {}\nEntry count: {}", &self.glossary_id, &self.name, &self.ready, &self.source_lang, &self.target_lang, &self.creation_time, &self.entry_count);
        let res = f.write_str(str.as_str());

        if let Ok(_) = res {
            Ok(())
        } else {
            Err(std::fmt::Error)
        }
    }
}

struct GlossaryRequest {
    // auth: 
}

impl GlossaryRequest {
    fn create_glossary_from_string(auth: &DeepLKey, name: String, source_lang: SourceLang, target_lang: TargetLang, entries: String) -> Result<Value, Box<dyn std::error::Error>> {
        let endpoint = "https://api-free.deepl.com/v2/glossaries";
        let mut params : Vec<String> = Vec::new();

        params.push(format!("name={}", name));
        params.push(format!("source_lang={}", source_lang));
        params.push(format!("target_lang={}", target_lang));
        params.push(format!("entries={}", entries));
        params.push("entries_format=tsv".to_string());

        let request = PostRequest {
            endpoint: endpoint, 
            auth: &auth.key, 
            params: params, 
        };

        let res = request.execute();

        if let Ok(v) = res {
            Ok(v)
        } else {
            panic!("I forgot how to pass error statements")
        }
    }

    fn get_glossaries(auth: &DeepLKey) -> Result<Vec<Glossary>, Box<dyn std::error::Error>> {
        let endpoint = "https://api-free.deepl.com/v2/glossaries";
        let mut glossaries : Vec<Glossary> = Vec::new();

        let request = GetRequest {
            endpoint: endpoint,
            auth: &auth.key,
            headers: None
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

    fn get_glossary(auth: &DeepLKey, glossary_id: String) -> Result<Glossary, Box<dyn std::error::Error>> {
        let endpoint = format!("https://api-free.deepl.com/v2/glossaries/{}", glossary_id);

        let request = GetRequest {
            auth: &auth.key, 
            endpoint: &endpoint.as_str(), 
            headers: None
        };

        let res = request.execute();

        if let Ok(g) = res {
            let glossary = Glossary::new(g);

            Ok(glossary)
        } else {
            panic!("Git gud");
        }
    }

    fn delete_glossary(auth: &DeepLKey, glossary_id: String) -> Result<(), Box<dyn std::error::Error>> {
        let endpoint = format!("https://api-free.deepl.com/v2/glossaries/{}", glossary_id);

        let request = DeleteRequest {
            endpoint: endpoint.as_str(), 
            auth: &auth.key, 
            headers: None, 
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

    fn get_glossary_entries(auth: &DeepLKey, glossary_id: String) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let endpoint = format!("https://api-free.deepl.com/v2/glossaries/{}/entries", glossary_id);

        let params = vec!["Accept: text/tab-separated-values"];

        let request = GetRequest {
            endpoint: endpoint.as_str(), 
            auth: &auth.key, 
            headers: Some(params), 
        };

        let res = request.execute_temp();

        if let Ok(map) = res {
            let mut hm : HashMap<String, String> = HashMap::new();

            let rows : Vec<&str> = map.split("\n").collect();
            for row in rows {
                let key_val : Vec<&str> = row.split("\t").collect();

                hm.insert(key_val[0].to_string(), key_val[1].to_string());
            }

            Ok(hm)
        } else {
            panic!("Git gud");
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let secret = read_secret("secret.txt");
    let key : String = format!("DeepL-Auth-Key {}", secret);

    let deepl_key = DeepLKey::new("secret.txt").unwrap();

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

    //glossary fun

    let glossary_request = GlossaryRequest::get_glossaries(&deepl_key);

    if let Ok(glossaries) = glossary_request {
        for glossary in glossaries {
            println!("NEW GLOSSARY: \n{}\n", glossary);
        }
    } else {
        println!("UH OH");
    }

    // get specific glossary
    let gid = "36ee07b0-ddc9-426e-a273-57b1dba75291".to_string();
    let glossary_request = GlossaryRequest::get_glossary(&deepl_key, gid);

    if let Ok(gloss) = glossary_request {
        println!("\n\n\nGLOSS: {}", gloss);
    } else {
        println!("Error getting glossary");
    }

    //delete specific glossary
    let gid = "2edfeefb-ace8-42c2-87ea-d3e8664e80ff".to_string();
    let delete_request = GlossaryRequest::delete_glossary(&deepl_key, gid);
    
    if let Ok(()) = delete_request {
        println!("YIPEEEE!");
    } else {
        println!("Failed to delete glossary");
    }

    //get glossary entries
    let gid = "98df5d07-9e20-4bba-84ca-e7c9f3d2add2".to_string();
    let get_request = GlossaryRequest::get_glossary_entries(&deepl_key, gid.clone());

    if let Ok(hash_map) = get_request {
        println!("Glossary {} entries:", gid);
        println!("{:?}", hash_map);
    } else {
        println!("NOOOOOOOOO");
    }

    //create a glossary
    // let glossary = String::from("Hello\tTschüss\nBye\tHallo");
    // let create_request = GlossaryRequest::create_glossary_from_string(&deepl_key, "Reverse".to_string(), SourceLang::En, TargetLang::De, glossary);

    // if let Ok(g) = create_request {
    //     println!("glossary_id: {}", g["glossary_id"]);
    //     println!("ready: {}", g["ready"]);
    //     println!("name: {}", g["name"]);
    //     println!("source_lang: {}", g["source_lang"]);
    //     println!("target_lang: {}", g["target_lang"]);
    //     println!("creation_time: {}", g["creation_time"]);
    //     println!("entry_count: {}", g["entry_count"]);
    // } else {
    //     println!("Failed to create glossary");
    // }

    Ok(())
}
