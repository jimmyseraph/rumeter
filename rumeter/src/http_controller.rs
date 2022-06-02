use async_trait::async_trait;
use rumeter_component::{samplers::http::{Method, HeaderMap, HttpSampler}, Controller, record::RecordData, Sampler};


#[derive(Clone)]
pub struct HttpController{
    method: Method,
    url: String,
    headers: HeaderMap,
    body: Option<String>,
}

impl HttpController {
    pub fn new(method: Method, url: &str, headers: HeaderMap, body: Option<String>) -> Self {
        Self { method, url: url.to_string(), headers, body }
    }
}

#[async_trait]
impl Controller for HttpController {
    async fn run(&self) -> Vec<RecordData> {
        let samp = HttpSampler::new(
            "Http sampler",
            &self.url, 
            self.method.clone(), 
            self.headers.clone(),
            self.body.clone(),
        );
        let re = samp.run().await;
        vec![re]
    }
}