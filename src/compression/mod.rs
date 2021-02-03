mod data_compression;
mod gzip_compression;

pub use data_compression::{ConstrainedDataCompression, DataCompression};
// TODO: should be removed, handled by constraints
pub use gzip_compression::{ConstrainedGzipCompression, GzipCompression};
