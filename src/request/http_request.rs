//! This module represents HTTP requests.  All DeepL API requests go through here to fetch data for the user.

use curl::easy::{Easy, List};
use serde_json::{Value};
use std::io::{Read};

/// For DeepL API requests, users can submit Get, Post, or Delete requests.
pub enum RequestType {
    Get, 
    Post, 
    Delete, 
}

/// An HttpRequest sends data to a DeepL API endpoint based on the user's desired specifications.
pub struct HttpRequest<'a> {
    /// RESTful URL for the request
    pub endpoint: &'a str, 

    /// Authorization header is needed for all requests
    pub auth: &'a String, 

    /// Other additional headers needed in the request
    pub headers: Option<Vec<String>>, 

    /// Potential data sent in the request
    pub body: Option<Vec<String>>, 

    /// Is this request GET, POST, or DELETE?
    pub request_type: RequestType, 

    /// The data type this request is expected to return
    pub response_type: HttpResponseType
}

/// This is similar to the "Accept" header used in HTTP requests.  For DeepL API calls, the returned data is either in JSON format or a String.
pub enum HttpResponseType {
    /// returned JSON value
    Json(Value), 

    /// Returned tab-separated values or any returned String
    Tsv(String), 
}

/// These are some of the expected potential errors to watch out for in a DeepL request.  Extra care should be taken to handle them.
#[derive(Debug, PartialEq, Eq)]
pub enum ApiError {
    /// Bad request.  Typically, the request is malformed.
    Http400, 

    /// Unauthorized.  Usually the authorization key is invalid.
    Http401, 

    /// Forbidden.  Your request makes sense, but you do not have the rights to carry out the request.
    Http403, 

    /// Not found.  For example, trying to delete a Glossary that doesn't exist.
    Http404, 

    /// Too many requests.  Repeated requests should be sent with some delay.
    Http429, 

    /// Quota exceeded.  You have used all the allotted characters in your API plan.
    Http456, 

    /// Temporary problems with the DeepL servers.  Like 429, please do not constantly resend requests.
    Http500Plus, 

    /// Of course, there are many other HTTP response codes.  If our API returns another response code not listed above (excluding 200 and 201), we leave it as an exercise to the reader to investigate.
    Unknown(u32), 

    /// The infamous HTTP 418.  This is only used here in expected impossible situations.
    Teapot
}

impl ApiError {
    /// Given an HTTP response code, this function returns a corresponding ApiError that can help explain the reason behind the code.  Of course, it can also produce non-errors like 200 or fake errors like 10 if the user desires.
    pub fn from_u32(code: u32) -> ApiError {
        match code {
            400 => ApiError::Http400, 
            401 => ApiError::Http401, 
            403 => ApiError::Http403, 
            404 => ApiError::Http404, 
            429 => ApiError::Http429, 
            456 => ApiError::Http456, 
            x if x >= 500 => ApiError::Http500Plus, 
            418 => ApiError::Teapot,
            _ => ApiError::Unknown(code)
        }
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::Http400 => f.write_str("Bad request"), 
            ApiError::Http401 => f.write_str("Unauthorized: Invalid API key"), 
            ApiError::Http403 => f.write_str("Forbidden"), 
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
    /// Execute an HttpRequest and receive the potential desired result or any errors that propagated.  All of our DeepL requests go through here.
    pub fn execute(&self) -> Result<HttpResponseType, Box<dyn std::error::Error>> {
        let mut easy = Easy::new();
        easy.url(&self.endpoint).unwrap();
        let mut vec_bytes : Vec<u8> = Vec::new();

        //no endpoint specified
        if &self.endpoint.len() == &0usize {
            return Err(Box::new(ApiError::Http400));
        }

        match &self.request_type {
            RequestType::Get => {
                easy.custom_request("GET")?;
            }, 
            RequestType::Post => {
                easy.custom_request("POST")?;

                // setup POST body
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

        // need &[u8] for the read_function() call later
        let mut vb = &vec_bytes[..];

        //add headers
        let mut list = List::new();
        let header = format!("Authorization: DeepL-Auth-Key {}", &self.auth);
        list.append(header.as_str()).unwrap();
        if let Some(h) = &self.headers {
            for head in h.into_iter() {
                list.append(head).unwrap();
            }
        }
        easy.http_headers(list).unwrap();

        //setup read and write channels
        let mut data = Vec::new();
        {
            let mut transfer = easy.transfer();

            //send information
            if vec_bytes.len() > 0 {
                transfer.read_function(|buf| {
                    Ok(vb.read(buf).unwrap_or(0))
                }).unwrap();
            }

            //receive information
            transfer.write_function(|d| {
                data.extend_from_slice(d);
        
                Ok(d.len())
            }).unwrap();
            transfer.perform().unwrap();
        }

        //check response code for potential error
        let response_code = easy.response_code().unwrap();
        if response_code != 200 && response_code != 201 {
            return Err(Box::new(ApiError::from_u32(response_code)));
        }

        //initial response should be an str
        let s = match std::str::from_utf8(&data) {
            Ok(v) => v,
            Err(e) => return Err(Box::new(e)),
        };

        //format str based on desired response type
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