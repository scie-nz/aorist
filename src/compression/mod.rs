mod data_compression;
mod gzip_compression;

pub use data_compression::{InnerDataCompression, DataCompression};
// TODO: should be removed, handled by constraints
pub use gzip_compression::{InnerGzipCompression, GzipCompression};
