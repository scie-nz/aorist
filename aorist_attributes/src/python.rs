use pyo3::prelude::*;
use aorist_util::init_logging;
use crate::attributes::*;

include!(concat!(env!("OUT_DIR"), "/python.rs"));
