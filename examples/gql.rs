use std::{time::Duration, fs::File, sync::{Arc, Mutex}};

use async_trait::async_trait;
use rumeter_component::{
    Controller,
    record::RecordData, 
    samplers::{http:: {HeaderValue, HeaderMap}, gql::GraphQLSampler}, 
    Sampler, 
    group::ThreadGroup, 
    output::file_output::FileOutput,
};
use serde::Serialize;
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

    let out = FileOutput::new(File::create("gql.rtl").unwrap());
    group.start(SimpleController::default(), Arc::new(Mutex::new(out))).await;
    info!("test finished");
    Ok(())
}

#[derive(Default, Clone)]
struct SimpleController;
#[async_trait]
impl Controller for SimpleController {
    async fn run(&self) -> Vec<RecordData> {
        let query = r#"
        query Login($email: String, $password: String){
            login(email: $email, password: $password){
                id
                username
                token
            }
        }
        "#;
        let vars = LoginVars{
            email: "liudao@testops.vip".to_string(),
            password: "123456".to_string(),
        };
        let mut headers = HeaderMap::new();
        headers.append("Access-Token", HeaderValue::from_static("123456"));
        let samp = GraphQLSampler::<LoginVars>::new(
            "test login", 
            "http://127.0.0.1:8888/api/graphql", 
            query, 
            headers, 
            Some(vars),
        );
        let re = samp.run().await;
        vec![re]
    }
}

#[derive(Serialize, Clone)]
struct LoginVars {
    email: String,
    password: String,
}