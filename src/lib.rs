pub mod deepl;
pub mod request;
pub mod dictionary;
pub mod my_memory;
pub mod text_generator;

pub mod text_manipulation{

    #[cfg(test)]
    mod tests {
        use std::{fs,path};
        use serde_json::{json, Value};
        // use crate::text_manipulation::{generate_paragraph, Corpus, write_paragraph_to_file, generate_text_for_language};
        use crate::deepl::*;
        use crate::request::http_request::{ApiError, HttpRequest, HttpResponseType, RequestType};
        use crate::request::glossary_request::{get_glossaries, get_glossary, delete_glossary, get_glossary_entries, create_glossary_from_string};
        use crate::request::translation_request::TranslationRequest;
        use crate::dictionary::get_meaning;
        use crate::my_memory::translate_q_langpair;
        use crate::text_generator::*;
        
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
        fn mymemory_nomral_operation() {
            // use crate::text_manipulation::{translate_q_langpair};
            let q = String::from("Hello");
            let langpair = String::from("en|it");
            let result = translate_q_langpair(q, langpair);
            //print!("{}", result);
            assert_eq!(result, "Ciao");
        }

        #[test]
        fn mymemory_invalid_dest_language() {
            // use crate::text_manipulation::{translate_q_langpair};
            let q = String::from("Hello");
            let langpair = String::from("en|sp");
            let result = translate_q_langpair(q, langpair);
            //print!("{}", result);
            assert_eq!(result, "'SP' IS AN INVALID TARGET LANGUAGE . EXAMPLE: LANGPAIR=EN|IT USING 2 LETTER ISO OR RFC3066 LIKE ZH-CN. ALMOST ALL LANGUAGES SUPPORTED BUT SOME MAY HAVE NO CONTENT");
        }

        #[test]
        fn mymemory_invalid_format() {
            // use crate::text_manipulation::{translate_q_langpair};
            let q = String::from("Hello");
            let langpair = String::from("ensp");
            let result = translate_q_langpair(q, langpair);
            //print!("{}", result);
            assert_eq!(result, "INVALID LANGUAGE PAIR SPECIFIED. EXAMPLE: LANGPAIR=EN|IT USING 2 LETTER ISO OR RFC3066 LIKE ZH-CN. ALMOST ALL LANGUAGES SUPPORTED BUT SOME MAY HAVE NO CONTENT");
        }

        #[test]
        fn mymemory_missing_string() {
            // use crate::text_manipulation::{translate_q_langpair};
            let q = String::from("");
            let langpair = String::from("en|it");
            let result = translate_q_langpair(q, langpair);
            //print!("{}", result);
            assert_eq!(result, "NO QUERY SPECIFIED. EXAMPLE REQUEST: GET?Q=HELLO&LANGPAIR=EN|IT");
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

        // Tests for the dictionary module functions.
        #[test]
        // Test function that checks if it has a valid file with secret API.
        fn valid_dict_secret_path() {
            let file_path = "dict_secret.txt";
            let path_exists = path::Path::new(file_path).exists();
            assert!(path_exists);
        }

        #[test]
        // Test function to check if it gives meaning for a valid word.
        fn test_meaning_of_valid_word() {
            let word = "ethernet";
            let meaning = &get_meaning(word).unwrap()[0];
            // println!("{:#?}", meaning);
            assert_eq!(meaning, 
                "a computer network architecture consisting of various specified local-area network protocols, devices, and connection methods")
        }

        #[test]
        // Test function to check if it gives meaning for a valid word.
        fn test_meaning_of_invalid_word() {
            let word = "asdjhtes";
            let meaning = &get_meaning(word);
            // println!("{:#?}", meaning);
            assert!(meaning.is_err());
        }

    }
}
