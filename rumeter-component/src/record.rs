use std::{fmt::Display, collections::HashMap};

pub const TITLE_NAMES: [&str; 17] = ["timeStamp", "elapsed", "label", "responseCode", "responseMessage", "threadName", "dataType", "success", "failureMessage", "bytes", "sentBytes", "grpThreads", "allThreads", "URL", "Latency", "IdleTime", "Connect"];

#[derive(Clone)]
pub struct RecordData {
    time_stamp: u128,
    elapsed: u64,
    label: String,
    response_code: u16,
    response_message: String,
    thread_name: String,
    data_type: String,
    success: bool,
    failure_message: Option<String>,
    bytes: u64,
    sent_bytes: u64,
    grp_threads: u32,
    all_threads: u32,
    url: String,
    latency: u64,
    idle_time: u64,
    connect: u64,
    response_result: Option<ResponseResult>,
}

#[derive(Clone)]
pub struct ResponseResult {
    response_headers: HashMap<String, String>,
    response_data: String,
}

impl ResponseResult {
    pub fn new(response_headers: HashMap<String, String>, response_data: String) -> Self {
        Self { response_headers, response_data }
    }

    pub fn get_headers(&self) -> HashMap<String, String> {
        self.response_headers.clone()
    }

    pub fn get_response_data(&self) -> String {
        self.response_data.clone()
    }
}

impl RecordData {
    pub fn new(
        time_stamp: u128,
        elapsed: u64,
        label: String,
        response_code: u16,
        response_message: String,
        thread_name: String,
        data_type: String,
        success: bool,
        failure_message: Option<String>,
        bytes: u64,
        sent_bytes: u64,
        grp_threads: u32,
        all_threads: u32,
        url: String,
        latency: u64,
        idle_time: u64,
        connect: u64,
        response_result: Option<ResponseResult>,
    ) -> Self {
        Self {
            time_stamp,
            elapsed,
            label,
            response_code,
            response_message,
            thread_name,
            data_type,
            success,
            failure_message,
            bytes,
            sent_bytes,
            grp_threads,
            all_threads,
            url,
            latency,
            idle_time,
            connect,
            response_result,
        }
    }

    pub fn thread_name(&mut self, thread_name: String) {
        self.thread_name = thread_name;
    }

    pub fn grp_threads(&mut self, grp_threads: u32) {
        self.grp_threads = grp_threads;
    }

    pub fn all_threads(&mut self, all_threads: u32) {
        self.all_threads = all_threads;
    }

    pub fn get_response_result(&self) -> Option<ResponseResult> {
        self.response_result.clone()
    }
}

impl Display for RecordData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}", 
            self.time_stamp, 
            self.elapsed,
            self.label,
            self.response_code,
            self.response_message,
            self.thread_name,
            self.data_type,
            self.success,
            self.failure_message.clone().unwrap_or("".to_string()),
            self.bytes,
            self.sent_bytes,
            self.grp_threads,
            self.all_threads,
            self.url,
            self.latency,
            self.idle_time,
            self.connect,
        )
    }
}