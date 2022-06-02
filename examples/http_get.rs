use std::{time::Duration, fs::File, sync::{Arc, Mutex}};
use async_trait::async_trait;
use rumeter_component::{
    group::ThreadGroup, 
    samplers::http::{HttpSampler, HeaderMap, HeaderValue, Method},
    output::file_output::FileOutput, 
    Controller, 
    record::RecordData, 
    Sampler, 
};
use tracing_subscriber::fmt::format::FmtSpan;
use tracing::*;

#[tokio::main]
async fn main() ->  Result<(), Box<dyn std::error::Error>> {
    // subscript tracing log
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    // define a ThreadGroup to run your controller in multiple thread
    // let group = ThreadGroup::new(10, Duration::from_secs(1), -1, Some(Duration::from_secs(300)));
    let group = ThreadGroup::new(10, Duration::from_secs(1), 10, None);

    // define the output file. this rtl file will record the load test data
    let out = FileOutput::new(File::create("http.rtl").unwrap());
    // start the load test
    group.start(SimpleController::default(), Arc::new(Mutex::new(out))).await;
    info!("test finished");
    Ok(())
}

// define your own Controller, must be implemented trait Default and Clone
#[derive(Default, Clone)]
pub struct SimpleController;

// your controller must implement trait Controller
#[async_trait]
impl Controller for SimpleController {
    // you can define all your load testing logic in this function named "run"
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