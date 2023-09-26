use ::serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RequestData {
    pub worker_id: u8,
    pub request_duration: Duration,
    pub request_data_size: usize,
    pub api_dependancy_duration: String,
    pub api_dependacy_consumption: String,
    pub remarks: String,
}
