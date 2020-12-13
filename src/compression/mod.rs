mod data_compression;
mod gzip_compression;

pub use self::data_compression::DataCompression;
// TODO: should be removed, handled by constraints
pub use self::gzip_compression::GzipCompression;
