mod attributes;

pub use attributes::*;

#[cfg(feature = "python")]
mod python;
pub use python::attributes_module;
