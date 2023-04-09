use curl::easy::{Easy, List};
use serde_json::{Value};
use std::io::{Read};

pub enum RequestType {
    Get, 
    Post, 
    Delete, 
}

pub struct HttpRequest<'a> {
    pub endpoint: &'a str, 
    pub auth: &'a String, 
    pub headers: Option<Vec<String>>, 
    pub body: Option<Vec<String>>, 
    pub request_type: RequestType, 
    pub response_type: HttpResponseType
}

pub enum HttpResponseType {
    Json(Value), 
    Tsv(String), 
}

impl<'a> HttpRequest<'a> {
    pub fn execute(&self) -> Result<HttpResponseType, Box<dyn std::error::Error>> {
        let mut easy = Easy::new();
        easy.url(&self.endpoint).unwrap();

        //let mut bytes: &[u8];
        let mut vec_bytes : Vec<u8> = Vec::new();
        //
        //println!("BYTES LEN: {}", bytes.len());

        match &self.request_type {
            RequestType::Get => {
                easy.custom_request("GET")?;
            }, 
            RequestType::Post => {
                easy.custom_request("POST")?;

                if let Some(b) = &self.body {
                    let body = b.join("&");
                    let bytes = body.as_bytes().clone();

                    for byte in bytes {
                        vec_bytes.push(byte.to_owned());
                    }

                    easy.post(true).unwrap();
                    easy.post_field_size(vec_bytes.len() as u64).unwrap();
                }
            }, 
            RequestType::Delete => {
                easy.custom_request("DELETE")?;
            }, 
        };

        //TODO: Change name
        let mut foo = &vec_bytes[..];

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

            if vec_bytes.len() > 0 {
                transfer.read_function(|buf| {
                    Ok(foo.read(buf).unwrap_or(0))
                }).unwrap();
            }

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

        //TODO: return Accept type
        // if s.len() > 0 {
        //     let v : Value = serde_json::from_str(s)?;
        //     Ok(v["message"].as_str().unwrap().to_string())
        // } else {
        //     Ok("".to_string())
        // }

        match &self.response_type {
            HttpResponseType::Json(_) => {
                let val = serde_json::from_str(s);
                match val {
                    Ok(j) => Ok(HttpResponseType::Json(j)), 
                    Err(e) => Err(Box::new(e))
                }
                // if let Ok(j) = val {
                //     Ok(HttpResponseType::Json(j))
                // } else {
                //     Err(Box::new(std::error::Error))
                // }
            }, 
            HttpResponseType::Tsv(_) => {
                Ok(HttpResponseType::Tsv(s.to_string()))
            }
        }

        // let val = serde_json::from_str(s);
        // if let Ok(x) = val {
        //     Ok(x)
        // } else {
        //     panic!("uh oh");
        // }
    }
}