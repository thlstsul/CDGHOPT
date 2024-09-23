use serde::{Deserialize, Serialize};

pub mod http;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Message {
    pub code: String,
    pub value: String,
}
