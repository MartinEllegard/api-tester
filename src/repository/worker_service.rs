use std::time::Duration;

use reqwest::Client;
use tokio::{
    sync::mpsc::{Receiver, Sender},
    time::timeout,
};

use crate::models::{request_data::RequestData, test_result::TestResult};

use crate::http_client::extract_request_data;

pub async fn receiver_handel(rx: Receiver<RequestData>, expected_amount: usize) -> TestResult {
    let mut data_store = Vec::<RequestData>::with_capacity(expected_amount);
    let mut channel = rx;
    loop {
        let data = timeout(Duration::from_secs(20), channel.recv()).await;
        match data {
            Ok(val) => {
                if val.is_some() {
                    data_store.push(val.unwrap());
                    println!("Datastore received result");
                }
            }
            Err(_) => {
                break;
            }
        }
    }

    TestResult::new(data_store)
}

pub async fn sender_handle(
    tx: Sender<RequestData>,
    client: Client,
    url: &str,
    runs: u16,
    id: u8,
) -> bool {
    let mut current_run = 1;

    while current_run <= runs {
        println!("Worker {} started Get action", id);
        let request = client.get(url);
        let extracted = extract_request_data(request, id).await;
        let _ = tx.send(extracted).await;
        current_run += 1;
        println!("Worker {} finnished Get action", id);
    }
    true
}
