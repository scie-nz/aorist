mod alluxio_location;
mod gcs_location;
mod hive_location;
mod remote_location;
mod web_location;

pub use alluxio_location::{AlluxioLocation, InnerAlluxioLocation};
pub use gcs_location::{InnerGCSLocation, GCSLocation};
pub use hive_location::{InnerHiveLocation, HiveLocation};
pub use remote_location::{InnerRemoteLocation, RemoteLocation};
pub use web_location::{InnerWebLocation, WebLocation};
