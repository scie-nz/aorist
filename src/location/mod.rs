mod alluxio_location;
mod gcs_location;
mod hive_location;
mod minio_location;
mod remote_location;
mod web_location;

pub use alluxio_location::{AlluxioLocation, InnerAlluxioLocation};
pub use minio_location::{MinioLocation, InnerMinioLocation};
pub use gcs_location::{GCSLocation, InnerGCSLocation};
pub use hive_location::{HiveLocation, InnerHiveLocation};
pub use remote_location::{InnerRemoteLocation, RemoteLocation};
pub use web_location::{InnerWebLocation, WebLocation};
