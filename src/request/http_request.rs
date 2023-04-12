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

#[derive(Debug)]
pub enum ApiError {
    Http401, 
    Http404, 
    Http429, 
    Http456, 
    Http500Plus, 
    Unknown(u32), 
    Teapot
}

impl ApiError {
    pub fn from_u32(code: u32) -> ApiError {
        match code {
            401 => ApiError::Http401, 
            404 => ApiError::Http404, 
            429 => ApiError::Http429, 
            456 => ApiError::Http456, 
            x if x >= 500 => ApiError::Http500Plus, 
            _ => ApiError::Unknown(code)
        }
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::Http401 => f.write_str("Unauthorized: Invalid API key"), 
            ApiError::Http404 => f.write_str("Not Found"),
            ApiError::Http429 => f.write_str("Too many requests"),
            ApiError::Http456 => f.write_str("Quota exceeded"),
            ApiError::Http500Plus => f.write_str("Temporary errors in DeepL service"), 
            ApiError::Unknown(u) => {
                let form = format!("HTTP {}", u);
                f.write_str(&form)
            }, 
            ApiError::Teapot => f.write_str("I'm a teapot!")
        }
    }
}

impl std::error::Error for ApiError{}

impl<'a> HttpRequest<'a> {
    pub fn execute(&self) -> Result<HttpResponseType, Box<dyn std::error::Error>> {
        let mut easy = Easy::new();
        easy.url(&self.endpoint).unwrap();
        let mut vec_bytes : Vec<u8> = Vec::new();

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

        let mut vb = &vec_bytes[..];

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
                    Ok(vb.read(buf).unwrap_or(0))
                }).unwrap();
            }

            transfer.write_function(|d| {
                data.extend_from_slice(d);
        
                Ok(d.len())
            }).unwrap();
            transfer.perform().unwrap();
        }

        let response_code = easy.response_code().unwrap();

        if response_code != 200 || response_code != 201 {
            return Err(Box::new(ApiError::from_u32(response_code)));
        }

        let s = match std::str::from_utf8(&data) {
            Ok(v) => v,
            Err(e) => return Err(Box::new(e)),
        };

        match &self.response_type {
            HttpResponseType::Json(_) => {
                let val = serde_json::from_str(s);
                match val {
                    Ok(j) => Ok(HttpResponseType::Json(j)), 
                    Err(e) => Err(Box::new(e))
                }
            }, 
            HttpResponseType::Tsv(_) => {
                Ok(HttpResponseType::Tsv(s.to_string()))
            }
        }
    }
}