use std::{fs, io::Write};

use crate::{Output, record::{TITLE_NAMES, RecordData}};

pub struct FileOutput {
    file: fs::File,
}

impl FileOutput {
    pub fn new(file: fs::File) -> Self {
        let mut f = file;
        let s = format!("{}\n", TITLE_NAMES.join(","));
        f.write_all(s.as_bytes()).unwrap();
        Self { file: f }
    } 
}

impl Output for FileOutput {
    fn write(&mut self, data: RecordData) {
        self.file.write_all(format!("{}\n", data).as_bytes()).unwrap();
    }
}