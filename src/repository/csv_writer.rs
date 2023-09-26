use std::{fmt::Display, fs::File};

use csv::Writer;

use crate::models::request_data::RequestData;

pub struct Error {}
impl Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub struct CsvWriter {
    writer: Writer<File>,
}
impl CsvWriter {
    pub fn new(file_path: &str) -> Result<Self, Error> {
        let try_make_writer = Writer::from_path(file_path);
        match try_make_writer {
            Ok(val) => Ok(CsvWriter { writer: val }),
            Err(_) => Err(Error {}),
        }
    }

    pub fn write_header(&mut self) {
        let _ = self.writer.write_record([
            "Worker id",
            "Request duration(ms)",
            "Request size(KB)",
            "API reported dependency duration(ms)",
            "API reported consumption",
            "Remark",
        ]);
    }

    pub fn write_data_line(&mut self, data: RequestData) {
        let _ = self.writer.write_record([
            &data.worker_id.to_string(),
            &data.request_duration.as_millis().to_string(),
            &(data.request_data_size / 1000).to_string(),
            &data.api_dependancy_duration,
            &data.api_dependacy_consumption,
            &data.remarks.to_string(),
        ]);
    }

    pub fn flush(&mut self) -> Result<(), std::io::Error> {
        self.writer.flush()
    }
}
