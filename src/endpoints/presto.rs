#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct PrestoConfig {
    server: String,
    httpPort: usize,
}

