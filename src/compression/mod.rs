mod data_compression;
mod gzip_compression;
mod zip_compression;

pub use data_compression::{DataCompression, InnerDataCompression};
// TODO: should be removed, handled by constraints
pub use gzip_compression::{GzipCompression, InnerGzipCompression};
pub use zip_compression::{InnerZipCompression, ZipCompression};
