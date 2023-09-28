use core::panic;
use models::{request_data::RequestData, test_result::TestResult};
use tokio::{
    sync::mpsc::{channel, Sender},
    task,
};

pub mod config;
pub mod http_client;
pub mod models;
mod repository;

#[tokio::main]
async fn main() {
    let client = http_client::create_client();

    let (tx, rx) = channel::<RequestData>(40);

    let number_of_runs = config::CONFIG.requests;
    let concurrency = config::CONFIG.workers;
    let runs_worker_for_workers = number_of_runs / concurrency;
    let url = match config::CONFIG.query_string.clone() {
        Some(val) => config::CONFIG.url.clone().unwrap() + val.as_str(),
        None => config::CONFIG.query_string.clone().unwrap(),
    };

    println!("Config - url: {}", url.clone());
    println!("Config - total runs: {}", number_of_runs);
    println!("Config - concurrency: {}", concurrency);
    println!("Config - run for each worker: {}", runs_worker_for_workers);

    let receiver_task = task::spawn(async move {
        repository::worker_service::receiver_handel(rx, number_of_runs as usize).await
    });

    let mut workers = task::JoinSet::<bool>::new();
    for index in 0..concurrency {
        let http_client = client.clone();
        let sender: Sender<RequestData> = tx.clone();
        let request_url = url.clone();
        let runs_worker = runs_worker_for_workers;
        workers.spawn(async move {
            repository::worker_service::sender_handle(
                sender,
                http_client,
                request_url.as_str(),
                runs_worker,
                index as u8,
            )
            .await
        });
    }

    let mut _workers_done = 0;

    while (workers.join_next().await).is_some() {
        _workers_done += 1;
    }

    let result = match receiver_task.await {
        Ok(val) => val,
        Err(_) => TestResult::new(Vec::<RequestData>::with_capacity(0)),
    };

    let mut csv_writer = match repository::csv_writer::CsvWriter::new("run_result.csv") {
        Ok(writer) => writer,
        Err(_) => panic!("Faile to create csv_writer"),
    };

    csv_writer.write_header();

    result.request_data.into_iter().for_each(|item| {
        csv_writer.write_data_line(item);
    });

    let success = csv_writer.flush();
    if success.is_ok() {
        println!("Successfully created csv: ./run_result.csv ");
    } else {
        println!("Failed to write csv");
    }
}
