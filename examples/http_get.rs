use std::{time::Duration, fs::File, sync::{Arc, Mutex}};
use async_trait::async_trait;
use rumeter_component::{
    group::ThreadGroup, 
    sampler::{HttpSampler, HeaderMap, HeaderValue, Method}, 
    output::file_output::FileOutput, 
    Controller, 
    record::RecordData, 
    Sampler, 
};
use tracing_subscriber::fmt::format::FmtSpan;
use tracing::*;

#[tokio::main]
async fn main() ->  Result<(), Box<dyn std::error::Error>> {

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let group = ThreadGroup::new(10, Duration::from_secs(1), -1, Some(Duration::from_secs(300)));
    // let group = ThreadGroup::new(10, Duration::from_secs(1), 10, None);

    let out = FileOutput::new(File::create("http.rtl").unwrap());
    group.start(SimpleController::default(), Arc::new(Mutex::new(out))).await;
    info!("test finished");
    Ok(())
}

#[derive(Default, Clone)]
pub struct SimpleController;

#[async_trait]
impl Controller for SimpleController {
    async fn run(&self) -> Vec<RecordData> {
        let mut headers = HeaderMap::new();
        headers.append("Access-Token", HeaderValue::from_static("123456"));
        let samp = HttpSampler::new(
            "test hello",
            "http://127.0.0.1:8088/hello", 
            Method::GET, 
            headers,
            None,
        );
        let re = samp.run().await;
        vec![re]
    }
}