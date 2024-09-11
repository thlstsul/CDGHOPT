use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Message {
    pub func: String,
    pub param: String,
}

impl Message {
    pub fn new(func: String, param: String) -> Self {
        Self { func, param }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub method: String,
    pub uri: String,
    pub header: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Request {
    pub fn new(method: String, uri: String, headers: Vec<(String, String)>, body: Vec<u8>) -> Self {
        let header = headers.into_iter().filter(|h| !h.0.is_empty()).collect();
        Self {
            method,
            uri,
            header,
            body,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    #[serde(with = "time::serde::timestamp")]
    pub done_date: OffsetDateTime,
    pub status: u16,
    pub header: HashMap<String, Vec<u8>>,
    pub body: Vec<u8>,
    pub elapsed_time: i32,
}
