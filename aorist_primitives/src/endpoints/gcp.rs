pub struct GCPConfig {
    pub use_default_credentials: bool,
    pub service_account_file: Option<String>,
    pub project_name: String,
    pub data_location: String,
}
