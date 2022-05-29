use std::collections::HashMap;

use async_trait::async_trait;
use serde::Serialize;
use tracing::*;
use crate::{Sampler, record::{RecordData, ResponseResult}};

use super::http::HeaderMap;


#[derive(Clone)]
pub struct GraphQLSampler<T: Serialize + Clone + Send + Sync>{
    label: String,
    endpoint: String,
    headers: HeaderMap,
    body: RequestBody<T>,
}

#[derive(Serialize, Clone)]
struct RequestBody<T: Serialize + Clone + Send> {
    query: String,
    variables: Option<T>,
}

impl <T: Serialize + Clone + Send + Sync> GraphQLSampler<T> {
    pub fn new(label: &str, endpoint: &str, query: &str, headers: HeaderMap, vars: Option<T>) -> Self {
        let body = RequestBody {
            query: query.to_string(),
            variables: vars,
        };
        Self { label: label.to_string(), endpoint: endpoint.to_string(), headers, body }
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
        ("POST".len() + self.endpoint.len() + "  HTTP/1.1\r\n".len()) as u32
    }

    fn request_body_size(&self) -> u32 {
        let s = serde_json::to_string(&self.body);
        (s.unwrap_or("".to_string()).len() + "\r\n".len()) as u32 
    }
}

#[async_trait]
impl <T: Serialize + Clone + Send + Sync> Sampler for GraphQLSampler<T> {
    async fn run(&self) -> RecordData {
        let client = reqwest::Client::new();
        let start_send_timestamp = chrono::Local::now();
        let resp = client.post(&self.endpoint).json(&self.body).headers(self.headers.clone()).send().await;
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
                    self.request_size() as u64,
                    0,
                    0,
                    self.endpoint.clone(),
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
                    self.request_size() as u64,
                    0,
                    0,
                    self.endpoint.clone(),
                    (finish_send_timestamp - start_send_timestamp).num_milliseconds() as u64,
                    0,
                    0,
                    None,
                )
            },
        }

    }
}

