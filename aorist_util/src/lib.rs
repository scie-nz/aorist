use serde_yaml::{from_str, Mapping, Value};
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use tracing;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{fmt, EnvFilter};
mod constraint;
pub use constraint::*;
mod core_structures;
pub use core_structures::*;
pub use aorist_error::{AResult, AoristError};

pub fn read_file(filename: &str) -> AResult<Vec<HashMap<String, Value>>> {
    let file = match File::open(filename) {
        Ok(x) => x,
        Err(_err) => panic!("Cannot find file {}.", filename),
    };
    let reader = BufReader::new(file);
    let mut buf: String = "".into();
    let mut result = Vec::new();
    for line in reader.lines() {
        let line_str = line?;
        if line_str == "---" {
            if buf.len() > 0 {
                let doc = match from_str(&buf) {
                    Ok(x) => x,
                    Err(err) => panic!(
                        "Error {:?} encountered when processing:\n---\n{}\n---\n.",
                        err, buf
                    ),
                };
                result.push(doc);
            }
            buf = "".into();
        } else {
            buf += "\n";
            buf += &line_str;
        }
    }
    if buf.len() > 0 {
        let doc = match from_str(&buf) {
            Ok(x) => x,
            Err(err) => panic!(
                "Error {:?} encountered when processing:\n---\n{}\n---\n.",
                err, buf
            ),
        };
        result.push(doc);
    }
    Ok(result)
}

pub fn get_raw_objects_of_type(
    raw_objects: &Vec<HashMap<String, Value>>,
    object_type: String,
) -> AResult<Vec<HashMap<String, Value>>> {
    let mut res: Vec<HashMap<String, Value>> = Vec::new();
    for x in raw_objects.iter() {
        let obj_type = x
            .get("type")
            .ok_or_else(|| {
                AoristError::UnexpectedNoneError("no 'type' field found on object".into())
            })?
            .as_str()
            .ok_or_else(|| {
                AoristError::CannotConvertJSONError("Could not convert JSON to string".into())
            })?
            .clone();
        if obj_type == object_type {
            let spec: Mapping = x
                .get("spec")
                .ok_or_else(|| {
                    AoristError::UnexpectedNoneError("no 'spec' field found on object".into())
                })?
                .as_mapping()
                .ok_or_else(|| {
                    AoristError::CannotConvertJSONError("Could not convert JSON to mapping".into())
                })?
                .clone();
            let mut map: HashMap<String, Value> = HashMap::new();
            for (k, v) in spec.into_iter() {
                let key = k.as_str().ok_or_else(|| {
                    AoristError::CannotConvertJSONError("Could not convert JSON to string".into())
                })?;
                map.insert(key.into(), v);
            }
            res.push(map);
        }
    }
    Ok(res)
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

