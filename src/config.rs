use core::panic;
use std::{env, str::FromStr};

use dotenvy::dotenv;

use once_cell::sync::Lazy;
use reqwest::header::{HeaderName, HeaderValue};

#[derive(Clone, Debug)]
pub struct ConfigHeader {
    pub key: HeaderName,
    pub value: HeaderValue,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub url: Option<String>,
    pub query_string: Option<String>,
    pub workers: u16,
    pub requests: u16,
    pub request_headers: Vec<ConfigHeader>,
}

pub static CONFIG: Lazy<Config> = Lazy::new(get_config);

fn get_config() -> Config {
    dotenv().unwrap();
    let mut base_config = Config::default();

    for (name, value) in env::vars() {
        match name.as_str() {
            x if x == "URL" => base_config.url = Some(value.clone()),
            x if x == "QUERYSTRINGS" => base_config.query_string = Some(value),
            x if x == "WORKERS" => base_config.workers = value.parse().unwrap_or(4),
            x if x == "REQUESTS" => base_config.requests = value.parse().unwrap_or(1000),
            x if x.contains("REQUESTHEADER") => {
                let split: Vec<&str> = value.as_str().split(' ').collect();

                if split.len() == 2 {
                    let header_name = match HeaderName::from_str(split[0]) {
                        Ok(val) => val,
                        Err(_) => continue,
                    };
                    let header_value = match HeaderValue::from_str(split[1]) {
                        Ok(val) => val,
                        Err(_) => continue,
                    };

                    base_config.request_headers.push(ConfigHeader {
                        key: header_name,
                        value: header_value,
                    });
                }
            }
            _ => (),
        };
    }

    if base_config.url.is_none() {
        panic!("Missing url")
    }

    base_config.clone()
}

impl Default for Config {
    fn default() -> Self {
        Config {
            url: None,
            query_string: None,
            workers: 4,
            requests: 100,
            request_headers: Vec::<ConfigHeader>::new(),
        }
    }
}
