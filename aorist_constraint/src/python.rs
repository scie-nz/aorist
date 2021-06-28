use pyo3::prelude::*;
use aorist_util::init_logging;
use crate::constraint::*;

include!(concat!(env!("OUT_DIR"), "/python.rs"));
