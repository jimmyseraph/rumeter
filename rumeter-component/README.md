# RuMeter

A load test platform for writing a load test script by rust. Just like JMeter, but it prefer using like SDK, not a GUI tool. It is:
* **Fast** RuMeter's zero-cost abstractions give you bare-metal performance.
* **Extensiable** RuMeter is easy to use and develop your own components.
* **Script** RuMeter is just an SDK, and the best way to use it is using in your script.

## Example
A basic HTTP API load test with RuMeter.

Make sure you add the dependence on your Cargo.toml:
```toml
[dependencies]
rumeter-component = "0.1.0"
```

Then, you should define your own controller first. Your controller must implement trait Controller:
```rust
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
```

Then, on your main.rs:
```rust,no_run
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
```

more examples can be found [here][examples].

[examples]: https://github.com/jimmyseraph/rumeter/tree/main/examples

## generate load test report
The rtl(RuMeter Test Log) file is a csv type file. You can use [JMeter] to generate the html style report like this:
```Shell
$ jmeter -g [your.rtl] -o [report_path]
```

[JMeter]: https://jmeter.apache.org

## Todo
Now only a few sampler has implemented. More commonly used samplers will implement in future version.

## License

This project is licensed under the [MIT license].

[MIT license]: https://github.com/jimmyseraph/rumeter/blob/main/LICENSE

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in RuMeter by you, shall be licensed as MIT, without any additional terms or conditions.