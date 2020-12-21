mod alluxio_location;
mod gcs_location;
mod hive_location;
mod remote_location;

// TODO: this should be handled by constraints, no need to expose
// in the future.
pub use self::alluxio_location::AlluxioLocation;
pub use self::gcs_location::GCSLocation;
pub use self::hive_location::HiveLocation;
pub use self::remote_location::RemoteLocation;
