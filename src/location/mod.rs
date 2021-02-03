mod alluxio_location;
mod gcs_location;
mod hive_location;
mod remote_location;
mod web_location;

pub use alluxio_location::{AlluxioLocation, ConstrainedAlluxioLocation};
pub use gcs_location::{ConstrainedGCSLocation, GCSLocation};
pub use hive_location::{ConstrainedHiveLocation, HiveLocation};
pub use remote_location::{ConstrainedRemoteLocation, RemoteLocation};
pub use web_location::{ConstrainedWebLocation, WebLocation};
