
use std::{time::Duration, fs::File, sync::{Arc, Mutex}};

use rumeter::http_controller::HttpController;
use rumeter_component::{group::ThreadGroup, output::file_output::FileOutput, samplers::http::{Method, HeaderMap, HeaderName, HeaderValue}};
use type_cli::CLI;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing::*;

#[derive(CLI)]
#[help = r#"Load test tool in rust."#]
enum ParameterOption{
    #[help = r#"Run http protocol load test.

    eg: rumeter http -m post -u http://127.0.0.1/api/login -H Content-Type=application/json -b "\{\"username\": \"liudao\", \"password\":\"123456\"\}" -n 10 -c 10 -l demo.rtl"#]
    Http {
        #[named(short="m")]
        #[optional]
        #[help = "Request method, default is GET."]
        method: Option<String>,

        #[named(short="u")]
        #[help = "Request url."]
        url: String,

        #[named(short="H")]
        #[optional]
        #[help = r#"Request header, split with '::'. eg: Content-Type=application/json::User-Agent=Mozilla/5.0"#]
        headers: Option<String>,

        #[named(short="b")]
        #[optional]
        #[help = "Request body"]
        body: Option<String>,

        #[named(short="n")]
        #[help = "Thread number"]
        number: u32,

        #[named(short="c")]
        #[optional]
        #[help = "Loop count, if duration is available, this option will be ignored."]
        count: Option<i32>,

        #[named(short="d")]
        #[optional]
        #[help = "Runing duration, in seconds."]
        duration: Option<u64>,

        #[named(short="l")]
        #[help = "Specify Rumeter test log file"]
        log: String,

    },

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    match ParameterOption::process() {
        ParameterOption::Http { method, url, headers, body, number, count, duration, log } => {
            let loop_num = count.unwrap_or(-1);
            let duration = match duration {
                Some(n) => Some(Duration::from_secs(n)),
                None => None,
            };
            let thread_group = ThreadGroup::new(number, Duration::from_secs(1), loop_num, duration);
            let file = File::create(&log).expect(format!("cannot create file {}", &log).as_str());
            let out = FileOutput::new(file);

            let method = Method::from(&method.unwrap_or("get".to_string())).unwrap();
            let mut header_map = HeaderMap::new();
            let mut header_str: Vec<String> = Vec::new();
            let mut header_key_value: Vec<String> = Vec::new();
            match headers {
                Some(s) => {
                    let header_vec: Vec<&str> = s.split("::").collect();
                    for item in header_vec {
                        header_str.push(item.to_string());
                    }
                    for item in header_str{
                        let key_value: Vec<&str> = item.split('=').collect();
                        if key_value.len() != 2 {
                            panic!("Cannot parse headers string");
                        }
                        for k_v in key_value {
                            header_key_value.push(k_v.to_string());
                        }
                        
                        header_map.insert(
                            HeaderName::from_bytes(header_key_value[0].as_bytes()).unwrap(), 
                            HeaderValue::from_bytes(header_key_value[1].as_bytes()).unwrap(),
                        );
                    }
                },
                None => {},
            };
            let controller = HttpController::new(method, &url, header_map, body);
            thread_group.start(controller, Arc::new(Mutex::new(out))).await;
            info!("test finished");
        },
    }

    Ok(())
}
