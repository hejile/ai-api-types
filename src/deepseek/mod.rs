use std::fmt::Display;

use serde::{Deserialize, Serialize};

pub mod chat_completion;

pub const BASE_URL: &str = "https://api.deepseek.com";
pub const BETA_BASE_URL: &str = "https://api.deepseek.com/beta";

#[derive(Debug, Serialize, Deserialize)]
pub enum Model {
    #[serde(rename = "deepseek-v4-flash")]
    V4Flash,
    #[serde(rename = "deepseek-v4-pro")]
    V4Pro,
}
