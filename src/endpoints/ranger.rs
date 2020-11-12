use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct RangerConfig {
    server: String,
    port: usize,
    user: String,
    password: String,
}

