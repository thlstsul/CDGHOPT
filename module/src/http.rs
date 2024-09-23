use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::Message;

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

impl TryFrom<Request> for Message {
    type Error = serde_json::Error;

    fn try_from(value: Request) -> Result<Self, Self::Error> {
        Ok(Self {
            code: "http_send".to_string(),
            value: serde_json::to_string(&value)?,
        })
    }
}

impl From<Response> for Message {
    fn from(value: Response) -> Self {
        let value = serde_json::to_string(&value);
        match value {
            Ok(value) => Self {
                code: "http_send".to_string(),
                value,
            },
            Err(e) => Self {
                code: "error".to_string(),
                value: e.to_string(),
            },
        }
    }
}
