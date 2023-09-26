use core::panic;
use std::{env, str::FromStr};

use dotenvy::dotenv;

use once_cell::sync::Lazy;
use reqwest::header::{HeaderName, HeaderValue};

#[derive(Clone, Debug)]
pub struct ConfigAuthHeader {
    pub key: HeaderName,
    pub value: HeaderValue,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub url: Option<String>,
    pub query_string: Option<String>,
    pub workers: u16,
    pub requests: u16,
    pub auth_headers: Vec<ConfigAuthHeader>,
}

pub static CONFIG: Lazy<Config> = Lazy::new(get_config);

fn get_config() -> Config {
    dotenv().unwrap();
    let mut base_config = Config::default();

    for (name, value) in env::vars() {
        match name.as_str() {
            x if x.contains("URL") => base_config.url = Some(value.clone()),
            x if x.contains("QUERYSTRINGS") => base_config.query_string = Some(value),
            x if x.contains("WORKERS") => base_config.workers = value.parse().unwrap_or(4),
            x if x.contains("REQUESTS") => base_config.requests = value.parse().unwrap_or(1000),
            x if x.contains("AUTHHEADER") => {
                let split: Vec<&str> = value.as_str().split(' ').collect();
                if split.len() == 2 {
                    let header_name = HeaderName::from_str(split[0]);
                    let header_value = HeaderValue::from_str(split[1]);

                    if header_name.is_ok() && header_value.is_ok() {
                        base_config.auth_headers.push(ConfigAuthHeader {
                            key: header_name.unwrap(),
                            value: header_value.unwrap(),
                        })
                    }
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
            auth_headers: Vec::<ConfigAuthHeader>::new(),
        }
    }
}
