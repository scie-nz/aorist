mod alluxio_location;
mod gcs_location;
mod minio_location;
mod on_premise_location;
mod remote_location;
mod web_location;

pub use alluxio_location::{AlluxioLocation, InnerAlluxioLocation};
pub use gcs_location::{GCSLocation, InnerGCSLocation};
pub use minio_location::{InnerMinioLocation, MinioLocation};
pub use on_premise_location::{InnerOnPremiseLocation, OnPremiseLocation};
pub use remote_location::{InnerRemoteLocation, RemoteLocation};
pub use web_location::{InnerWebLocation, WebLocation};
