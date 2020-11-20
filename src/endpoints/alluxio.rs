#![allow(non_snake_case)]
use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Getters, Setters)]
pub struct AlluxioConfig {
    #[getset(get = "pub", set = "pub")]
    server: String,
    #[getset(get = "pub", set = "pub")]
    rpcPort: usize,
    #[getset(get = "pub", set = "pub")]
    apiPort: usize,
}
