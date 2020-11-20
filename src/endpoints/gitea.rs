use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct GiteaConfig {
    server: String,
    port: usize,
    token: String,
}
