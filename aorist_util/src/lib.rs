use serde_yaml::{from_str, Value};
use std::collections::HashMap;
use std::fs;
use tracing;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{fmt, EnvFilter};


pub fn read_file(filename: &str) -> Vec<HashMap<String, Value>> {
    let s = fs::read_to_string(filename).unwrap();
    s.split("\n---\n")
        .filter(|x| x.len() > 0)
        .map(|x| from_str(x).unwrap())
        .collect()
}
pub fn get_raw_objects_of_type(
    raw_objects: &Vec<HashMap<String, Value>>,
    object_type: String,
) -> Vec<HashMap<String, Value>> {
    raw_objects
        .into_iter()
        .filter(|x| x.get("type").unwrap().as_str().unwrap() == object_type)
        .map(|x| {
            x.get("spec")
                .unwrap()
                .as_mapping()
                .unwrap()
                .into_iter()
                .map(|(k, v)| (k.as_str().unwrap().into(), v.clone()))
                .collect()
        })
        .collect::<Vec<HashMap<String, Value>>>()
}
/// Configures logging for Aorist based on a LOG_LEVEL envvar.
/// Valid values are: error/warn/info/debug/trace/off (default: info)
pub fn init_logging() {
    let filter_layer = EnvFilter::try_from_env("LOG_LEVEL")
        .or_else(|_| EnvFilter::try_new("info"))
        .expect("Failed to initialize filter layer");

    tracing::subscriber::set_global_default(
        tracing_subscriber::Registry::default()
            .with(filter_layer)
            .with(fmt::layer()),
    )
    .expect("Failed to set default subscriber");
}
