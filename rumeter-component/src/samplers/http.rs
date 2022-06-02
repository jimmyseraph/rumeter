use std::fmt;
use std::{collections::HashMap, error::Error};

use tracing::*;
use async_trait::async_trait;

use crate::{Sampler, record::{RecordData, ResponseResult}};

pub type HeaderMap = reqwest::header::HeaderMap;
pub type HeaderValue = reqwest::header::HeaderValue;
pub type HeaderName = reqwest::header::HeaderName;

#[derive(Clone)]
pub struct HttpSampler {
    label: String,
    url: String,
    method: Method,
    headers: HeaderMap,
    body: Option<String>,
}

#[derive(Debug)]
pub struct RumeterErr {
    message: String,
}

impl RumeterErr {
    pub fn new(message: &str) -> Self {
        Self { message: message.to_string() }
    }
}

impl fmt::Display for RumeterErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An Error Occurred, {}", self.message) // user-facing output
    }
}

impl Error for RumeterErr {
    
}

#[derive(Clone)]
pub enum Method {
    GET,
    POST,
    PUT,
}

impl Method {
    pub fn from(m: &str) -> Result<Self, Box<dyn Error>> {
        if m.eq_ignore_ascii_case("get") {
            Ok(Method::GET)
        } else if m.eq_ignore_ascii_case("post") {
            Ok(Method::POST)
        } else if m.eq_ignore_ascii_case("PUT") {
            Ok(Method::PUT)
        } else {
            Err(Box::new(RumeterErr::new("method not supported")))
        }
    }

    pub fn len(&self) -> u32{
        match self {
            Method::GET => 3,
            Method::POST => 4,
            Method::PUT => 3,
        }
    }
}

impl HttpSampler {
    pub fn new(label: &str, url: &str, method: Method, headers: HeaderMap, body: Option<String>) -> Self {
        Self { label: label.to_string(), url: url.to_string(), method, headers, body }
    }

    fn request_size(&self) -> u32 {
        self.request_line_size() + self.request_headers_size() + self.request_body_size()
    }

    fn request_headers_size(&self) -> u32 {
        let mut size = 0u32;
        for (key, value) in self.headers.clone() {
            match key {
                Some(header_name) => {
                    size = size + (header_name.to_string().len() + value.len() + ":\r\n".len()) as u32;
                },
                None => {},
            }
        }
        size
    }

    fn request_line_size(&self) -> u32 {
        self.method.len() + (self.url.len() + "  HTTP/1.1\r\n".len()) as u32
    }

    fn request_body_size(&self) -> u32 {
        (self.body.clone().unwrap_or("".to_string()).len() + "\r\n".len()) as u32 
    }
}

#[async_trait]
impl Sampler for HttpSampler {
    async fn run(&self) -> RecordData{
        let client = reqwest::Client::new();
        let s = self.clone();
        let start_send_timestamp = chrono::Local::now();
        let resp = match self.method {
            Method::GET => client.get(s.url.clone()).headers(s.headers.clone()).send().await,
            Method::POST => client.post(s.url.clone()).headers(s.headers.clone()).body(s.body.clone().unwrap()).send().await,
            Method::PUT => client.put(s.url.clone()).headers(s.headers.clone()).body(s.body.clone().unwrap()).send().await,
        };
        
        let finish_send_timestamp = chrono::Local::now();
        match resp {
            Ok(r) => {
                let data_type = String::from("text");
                let code = r.status().as_u16();
                let resp_msg = r.status().canonical_reason().unwrap_or("Unknown");
                let success = code < 400u16;
                let fail_msg = if success {
                    None
                } else {
                    Some(resp_msg.to_string())
                };
                let mut resp_headers: HashMap<String, String> = HashMap::new();
                for (h_key, h_val) in r.headers() {
                    resp_headers.insert(h_key.to_string(), h_val.to_str().unwrap().to_string());
                }

                let resp_body = r.text().await.unwrap_or("".to_string());

                RecordData::new(
                    start_send_timestamp.timestamp_millis() as u128,
                    (finish_send_timestamp - start_send_timestamp).num_milliseconds() as u64,
                    self.label.clone(),
                    code,
                    resp_msg.into(),
                    "".to_string(),
                    data_type,
                    success,
                    fail_msg,
                    resp_body.len() as u64,
                    s.request_size() as u64,
                    0,
                    0,
                    s.url,
                    (finish_send_timestamp - start_send_timestamp).num_milliseconds() as u64,
                    0,
                    0,
                    Some(ResponseResult::new(resp_headers, resp_body)),
                )

            },
            Err(e) => {
                error!("failed! --> {}", e.to_string());
                RecordData::new(
                    start_send_timestamp.timestamp_millis() as u128,
                    (finish_send_timestamp - start_send_timestamp).num_milliseconds() as u64,
                    self.label.clone(),
                    0,
                    "no data".to_string(),
                    "".to_string(),
                    "no data".to_string(),
                    false,
                    Some(e.to_string()),
                    0u64,
                    s.request_size() as u64,
                    0,
                    0,
                    s.url,
                    (finish_send_timestamp - start_send_timestamp).num_milliseconds() as u64,
                    0,
                    0,
                    None,
                )
            },
        }
    }

}