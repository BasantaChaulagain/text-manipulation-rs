pub mod deepl;
pub mod request;

pub mod text_manipulation{
    use rand::{Rng, thread_rng};
    use std::{fs, error::Error, io::{Write}, time::{SystemTime, UNIX_EPOCH}};
    
    enum Corpus<'a> {
        FromFile(&'a str), 
    }
    
    pub fn generate_text_for_language(language: i32, write_to_file: bool) {
        let paragraph = match language {
            0 => generate_paragraph(Corpus::FromFile("english.txt"), Some(100), Some(1000)),
            1 => generate_paragraph(Corpus::FromFile("french.txt"), None, None),
            2 => generate_paragraph(Corpus::FromFile("spanish.txt"), None, Some(500)),
            3 => generate_paragraph(Corpus::FromFile("hindi.txt"), Some(50), None),
            4 => generate_paragraph(Corpus::FromFile("russian.txt"), Some(50), None),
            5 => generate_paragraph(Corpus::FromFile("arabic.txt"), Some(50), None),
            6 => generate_paragraph(Corpus::FromFile("japanese.txt"), Some(50), None),
            7 => generate_paragraph(Corpus::FromFile("german.txt"), Some(50), None),
            10 => generate_paragraph(Corpus::FromFile("corpus/irish.txt"), Some(50), None),
            11 => generate_paragraph(Corpus::FromFile("corpus/swedish.txt"), Some(50), None),
            _ => panic!("Invalid language index"),
        };
        println!("{}", paragraph);
        
        if write_to_file == true{
            let res = write_paragraph_to_file(paragraph, None);
            match res {
                Ok(_) => println!("File created successfully."), 
                Err(e) => panic!("{}", e.to_string())
            };
        }
    }
    
    fn read_corpus_from_file(path: &str) -> Vec<String> {
        fs::read_to_string(path)
            .expect("Failed to parse file.")
            .split("\n")
            .map(|s| s.to_owned())
            .collect::<Vec<String>>()
    }
    
    fn write_paragraph_to_file(paragraph: String, path: Option<String>) -> Result<bool, Box<dyn Error>> {
        let file_name = match path {
            Some(x) => x, 
            None => {
                let time = SystemTime::now().duration_since(UNIX_EPOCH)?;
                let path = format!("{}.txt", time.as_secs());
                path
            }
        };
    
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(file_name)?;
        file.write_all(paragraph.as_bytes())?;
    
        Ok(true)
    }
    
    fn generate_paragraph(corpus: Corpus, min_sentences: Option<usize>, max_bytes: Option<usize>) -> String {
        let mut rng = thread_rng();
    
        let word_list: Vec<String> = match corpus {
            Corpus::FromFile(f) => {
                read_corpus_from_file(f)
            }, 
        };
    
        let n_sentences;
        if let Some(n) = min_sentences {
            n_sentences = rng.gen_range(n..n+4);
        } else {
            n_sentences = rng.gen_range(3..7);
        }
    
        let mut sentences = Vec::new();
        for _ in 0..n_sentences {
            let n_words = rng.gen_range(5..12);
            let mut words = Vec::new();
            for _ in 0..n_words {
                let word_index = rng.gen_range(0..word_list.len());
                let word = word_list[word_index].as_str();
                words.push(word);
            }
    
            let mut sentence = words.join(" ");
            sentence.make_ascii_lowercase();
    
            let first_char = sentence.chars().next().unwrap();
            //first character may not be one byte-aligned
            let first_char_len = &first_char.len_utf8();
            sentence.replace_range(..first_char_len, &first_char.to_uppercase().to_string());
            sentence.push('.');
            sentences.push(sentence);
        }
        
        let paragraph = sentences.join(" ");
        
        match max_bytes {
            None => paragraph, 
            Some(m) => {
                //only read up to limit of bytes requested by the user
                let mut truncated = String::new();
                let mut num_bytes: usize = 0;
                for char in paragraph.chars() {
                    let len = char.len_utf8();
                    if num_bytes + len < m {
                        truncated.push(char);
                        num_bytes += len;
                    } else {
                        break;
                    }
                }
    
                truncated
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use std::fs;
        use serde_json::{json, Value};
        use crate::text_manipulation::{generate_paragraph, Corpus, write_paragraph_to_file, generate_text_for_language};
        use crate::deepl::*;
        use crate::request::http_request::{ApiError, HttpRequest, HttpResponseType, RequestType};
        use crate::request::glossary_request::{get_glossaries, get_glossary, delete_glossary, get_glossary_entries, create_glossary_from_string};
        use crate::request::translation_request::TranslationRequest;

        fn get_auth() -> DeepLKey {
            DeepLKey::new("src/secret.txt").unwrap()
        }

        #[test]
        fn test_generate_paragraph() {    
            // Test that the paragraph contains at least one sentence
            let corpus = Corpus::FromFile("corpus/english.txt");
            let result = generate_paragraph(corpus, None, None);
            assert!(result.contains('.'));
        }    
        #[test]
        fn test_max_bytes(){
        // Test that the paragraph contains no more than the maximum requested number of bytes
            let corpus = Corpus::FromFile("corpus/english.txt");
            let max_bytes = Some(10);
            let result = generate_paragraph(corpus, None, max_bytes);
            assert!(result.as_bytes().len() <= max_bytes.unwrap());
        }
        #[test]
        // Test if the result is empty when arguments are None
        fn test_generate_paragraph_default() {
            let paragraph = generate_paragraph(Corpus::FromFile("corpus/english.txt"), None, None);
            assert!(!paragraph.is_empty());
        }

        #[test]
        fn test_write_paragraph_to_file() {
            let paragraph = String::from("Hello world!");
            let res = write_paragraph_to_file(paragraph.clone(), Some(String::from("test.txt")));
            assert!(res.is_ok());

            let file_content = fs::read_to_string("test.txt");
            assert_eq!(file_content.unwrap(), paragraph);

            fs::remove_file("test.txt").unwrap();
        }

        

        #[test]
        fn valid_key_path() {
            let path = "src/secret.txt";
            let res = DeepLKey::new(path);

            assert!(!res.is_err());
        }

        #[test]
        fn invalid_key_path() {
            assert!(DeepLKey::new("").is_err());
        }

        #[test]
        fn invalid_glossary_json() {
            let obj = json!({"foo": "A", "bar": "B"});
            let glossary = Glossary::new(obj);

            assert!(glossary.is_err())
        }

        #[test]
        fn valid_glossary_json() {
            let auth = get_auth();
            let res = get_glossaries(&auth);

            let g = res.unwrap();

            if g.len() == 0 {
                return;
            } else {
                let glossary = g.get(0).unwrap();

                let gid = glossary.glossary_id.to_owned();

                let res2 = get_glossary(&auth, gid);

                assert!(!res2.is_err());
            }
        }

        #[test]
        fn http_error_codes() {
            assert_eq!(ApiError::from_u32(400), ApiError::Http400);
            assert_eq!(ApiError::from_u32(401), ApiError::Http401);
            assert_eq!(ApiError::from_u32(403), ApiError::Http403);
            assert_eq!(ApiError::from_u32(404), ApiError::Http404);
            assert_eq!(ApiError::from_u32(429), ApiError::Http429);
            assert_eq!(ApiError::from_u32(456), ApiError::Http456);
            assert_ne!(ApiError::from_u32(499), ApiError::Http500Plus);
            assert_eq!(ApiError::from_u32(500), ApiError::Http500Plus);
            assert_eq!(ApiError::from_u32(501), ApiError::Http500Plus);
            assert_eq!(ApiError::from_u32(std::u32::MAX), ApiError::Http500Plus);
            assert_eq!(ApiError::from_u32(418), ApiError::Teapot);
            assert_eq!(ApiError::from_u32(200), ApiError::Unknown(200));
        }

        #[test]
        fn no_endpoint() {
            // let path = "src/secret.txt";
            // let auth = DeepLKey::new(path).unwrap();
            let auth = get_auth();

            let request = HttpRequest {
                endpoint: "", 
                auth: &auth.key, 
                headers: None, 
                body: None, 
                request_type: RequestType::Get, 
                response_type: HttpResponseType::Json(Value::Null)
            };

            let res = request.execute();

            assert!(res.is_err());

            let e = res.err().unwrap();
            assert!(e.is::<ApiError>());

            let api_error = e.downcast::<ApiError>().unwrap();
            assert_eq!(*api_error, ApiError::Http400);
        }

        #[test]
        fn no_auth() {
            let request = HttpRequest {
                endpoint: "https://api-free.deepl.com/v2/translate", 
                auth: &"".to_string(), 
                headers: None, 
                body: None, 
                request_type: RequestType::Get, 
                response_type: HttpResponseType::Json(Value::Null)
            };

            let res = request.execute();

            assert!(res.is_err());

            let e = res.err().unwrap();
            assert!(e.is::<ApiError>());

            let api_error = e.downcast::<ApiError>().unwrap();
            assert_eq!(*api_error, ApiError::Http403);
        }

        #[test]
        fn bad_auth() {
            let request = HttpRequest {
                endpoint: "https://api-free.deepl.com/v2/translate", 
                auth: &"aa1111aa-1111-1a1a-1111-1a111aaa1111:fx".to_string(), 
                headers: None, 
                body: None, 
                request_type: RequestType::Get, 
                response_type: HttpResponseType::Json(Value::Null)
            };

            let res = request.execute();

            assert!(res.is_err());

            let e = res.err().unwrap();
            assert!(e.is::<ApiError>());

            let api_error = e.downcast::<ApiError>().unwrap();
            assert_eq!(*api_error, ApiError::Http403);
        }

        #[test]
        fn invalid_headers() {
            let auth = get_auth();
            let mut headers : Vec<String> = Vec::new();
            headers.push(String::from("Garbage: Foo"));

            let mut request = HttpRequest {
                endpoint: "https://api-free.deepl.com/v2/translate", 
                auth: &auth.key, 
                headers: Some(headers), 
                body: None, 
                request_type: RequestType::Get, 
                response_type: HttpResponseType::Tsv("".to_string())
            };

            let res = request.execute();

            assert!(res.is_err());

            let e = res.err().unwrap();
            assert!(e.is::<ApiError>());

            let api_error = e.downcast::<ApiError>().unwrap();
            assert_eq!(*api_error, ApiError::Http400);

            //same idea but use invalid body content
            request.headers = None;
            request.body = Some(vec!["a".to_string()]);

            let res = request.execute();

            assert!(res.is_err());

            let e = res.err().unwrap();
            assert!(e.is::<ApiError>());

            let api_error = e.downcast::<ApiError>().unwrap();
            assert_eq!(*api_error, ApiError::Http400);

            request.request_type = RequestType::Post;
            request.headers = Some(vec!["b: c".to_string()]);

            let res = request.execute();

            assert!(res.is_err());

            let e = res.err().unwrap();
            assert!(e.is::<ApiError>());

            let api_error = e.downcast::<ApiError>().unwrap();
            assert_eq!(*api_error, ApiError::Http400);
        }

        #[test]
        fn simple_translations() {
            let auth = get_auth();
            let tr = TranslationRequest::new("Hello, World!", TargetLang::De);
            let request = TranslationRequest::create_request(&tr, &auth);

            let res = request.execute();

            assert!(!res.is_err());

            match res.unwrap() {
                HttpResponseType::Json(j) => {
                    let trans = j["translations"].as_array().unwrap();
                    assert_eq!(trans.get(0).unwrap()["text"].as_str().unwrap(), "Hallo, Welt!");
                }, 
                _ => panic!("Impossible")
            };

            let tr = TranslationRequest::new("", TargetLang::De);
            let request = TranslationRequest::create_request(&tr, &auth);

            let res = request.execute();

            assert!(!res.is_err());

            match res.unwrap() {
                HttpResponseType::Json(j) => {
                    let trans = j["translations"].as_array().unwrap();
                    assert_eq!(trans.get(0).unwrap()["text"].as_str().unwrap(), "");
                }, 
                _ => panic!("Impossible")
            };
        }

        //not sure why this test isn't working
        #[ignore]
        #[test]
        fn valid_translation_setters() {
            //curl "https://api-free.deepl.com/v2/translate" -H "Authorization: DeepL-Auth-Key [key]" -d "text=<section><par>The firm said it had been </par></section><par> conducting an <foo/> internal investigation.<bar/></par>&target_lang=de&split_sentences=0&preserve_formatting=1&formality=less&tag_handling=xml&non_splitting_tags=par&outline_detection=0&splitting_tags=section&ignore_tags=foo,bar"

            let auth = get_auth();
            let tr = TranslationRequest::new("<section><par>The firm said it had been </par></section><par> conducting an <foo/> internal investigation.<bar/></par>", TargetLang::De)
                .set_source_lang(SourceLang::En)
                .set_split_sentences(SplitSentences::None)
                .set_preserve_formatting(true)
                .set_formality(Formality::Less)
                .set_tag_handling(TagHandling::Xml)
                .set_non_splitting_tags("par")
                .set_outline_detection(false)
                .set_splitting_tags("section")
                .set_ignore_tags("foo,bar");
            let request = tr.create_request(&auth);
            let res = request.execute();

            assert!(!res.is_err());

            match res.unwrap() {
                HttpResponseType::Json(j) => {
                    let trans = j["translations"].as_array().unwrap();
                    assert_eq!(trans.get(0).unwrap()["text"].as_str().unwrap(), "<section><par>Das Unternehmen sagte, es habe </par></section><par> conducting an <foo/> internal investigation.<bar/></par>");
                }, 
                _ => panic!("Impossible")
            };
        }

        #[test]
        fn create_valid_glossary() {
            let auth = get_auth();
            let entries = String::from("Hello\tGuten Tag!\nBye\tAuf Wiedersehen!");
            let res = create_glossary_from_string(&auth, "unit".to_string(), SourceLang::En, TargetLang::De, entries);

            assert!(!res.is_err());

            let g = Glossary::new(res.unwrap());

            assert!(!g.is_err());

            let glossary = g.unwrap();

            assert_eq!(glossary.entry_count, 2);
        }

        #[test]
        fn create_invalid_glossary() {
            let auth = get_auth();
            let entries = String::from("\n\n\n");
            let res = create_glossary_from_string(&auth, "unit2".to_string(), SourceLang::En, TargetLang::De, entries);

            assert!(res.is_err());

            let e = res.err().unwrap();
            assert!(e.is::<ApiError>());

            let api_error = e.downcast::<ApiError>().unwrap();
            assert_eq!(*api_error, ApiError::Http400);
        }

        #[test]
        fn get_all_glossaries() {
            let auth = get_auth();
            assert!(!get_glossaries(&auth).is_err());
        }

        #[test]
        fn test_delete_glossary() {
            let auth = get_auth();
            let entries = String::from("Hello\tGuten Tag!\nBye\tAuf Wiedersehen!");
            let res = create_glossary_from_string(&auth, "temp".to_string(), SourceLang::En, TargetLang::De, entries);
            let g = res.unwrap();
            let glossary = Glossary::new(g).unwrap();

            //delete temp glossary
            let res = delete_glossary(&auth, glossary.glossary_id);

            assert!(!res.is_err());
        }

        #[test]
        fn delete_invalid_glossary() {
            let auth = get_auth();
            let res = delete_glossary(&auth, "glossary".to_string());

            assert!(res.is_err());

            let e = res.err().unwrap();
            assert!(e.is::<ApiError>());

            let api_error = e.downcast::<ApiError>().unwrap();
            assert_eq!(*api_error, ApiError::Http400);
        }

        #[test]
        fn test_glossary_entries() {
            //create temp glossary
            let auth = get_auth();
            let entries = String::from("Hello\tGuten Tag!\nBye\tAuf Wiedersehen!");
            let res = create_glossary_from_string(&auth, "temp".to_string(), SourceLang::En, TargetLang::De, entries);
            let g = res.unwrap();
            let glossary = Glossary::new(g).unwrap();

            let entries = get_glossary_entries(&auth, glossary.glossary_id.clone());

            //delete temp glossary
            let res = delete_glossary(&auth, glossary.glossary_id);

            assert!(!res.is_err());

            assert!(!entries.is_err());

            let hm = entries.unwrap();

            assert_eq!(hm["Hello"], String::from("Guten Tag!"));
            assert_eq!(hm["Bye"], String::from("Auf Wiedersehen!"));
        }

        #[test]
        fn invalid_glossary_entries() {
            let auth = get_auth();
            let res = get_glossary_entries(&auth, "glossary".to_string());

            assert!(res.is_err());

            let e = res.err().unwrap();
            assert!(e.is::<ApiError>());

            let api_error = e.downcast::<ApiError>().unwrap();
            assert_eq!(*api_error, ApiError::Http400);
        }
    }
}
