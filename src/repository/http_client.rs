use std::time::Duration;

use reqwest::{
    header::{self, HeaderMap},
    Client, RequestBuilder,
};
use tokio::time::Instant;

use crate::{config::CONFIG, models::request_data::RequestData};

pub fn create_client() -> Client {
    let mut header_map = HeaderMap::new();
    //Add acceptance headers for both encoding and contenttype
    header_map.append(
        header::ACCEPT_ENCODING,
        "br;q=1.0, gzip;q=0.8, *;q=0.1".parse().unwrap(),
    );
    header_map.append(header::ACCEPT, "application/json".parse().unwrap());
    for header in CONFIG.auth_headers.clone() {
        header_map.append(header.key, header.value.clone());
    }

    //Build Client
    let builder = Client::builder()
        .default_headers(header_map)
        .timeout(Duration::from_secs(60))
        .connect_timeout(Duration::from_secs(5));

    builder.build().unwrap()
}

pub async fn extract_request_data(built_request: RequestBuilder, id: u8) -> RequestData {
    let timer = Instant::now();
    let request = built_request.send().await;
    match request {
        Ok(response) => {
            let headers = response.headers().clone();
            let time_taken = timer.elapsed();

            let querytime_header = headers.get("duration");
            let querytime = match querytime_header {
                Some(val) => val.to_str().unwrap_or("").to_string(),
                None => "".to_string(),
            };
            let request_charge_header = headers.get("consumption");
            let request_charge = match request_charge_header {
                Some(val) => val.to_str().unwrap_or("").to_string(),
                None => "".to_string(),
            };

            let response_size = response.bytes().await.unwrap_or_default();
            RequestData {
                worker_id: id,
                request_duration: time_taken,
                request_data_size: response_size.len(),
                api_dependancy_duration: querytime,
                api_dependacy_consumption: request_charge,
                remarks: String::new(),
            }
        }
        Err(_) => RequestData {
            worker_id: id,
            request_duration: Duration::new(0, 0),
            request_data_size: 0,
            api_dependancy_duration: String::new(),
            api_dependacy_consumption: String::new(),
            remarks: "Failed to get response".to_string(),
        },
    }
}
