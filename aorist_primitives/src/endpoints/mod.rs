mod alluxio;
mod aws;
mod dask;
mod gcp;
mod gitea;
mod linz;
mod minio;
mod pdal;
mod postgres;
mod presto;
mod ranger;

pub use alluxio::*;
pub use aws::*;
pub use dask::*;
pub use gcp::*;
pub use gitea::*;
pub use linz::*;
pub use minio::*;
pub use pdal::*;
pub use postgres::*;
pub use presto::*;
pub use ranger::*;

#[cfg(feature = "python")]
pub fn endpoints_module(
    _py: pyo3::prelude::Python,
    m: &pyo3::prelude::PyModule,
) -> pyo3::prelude::PyResult<()> {
    m.add_class::<AlluxioConfig>()?;
    m.add_class::<AWSConfig>()?;
    m.add_class::<DaskConfig>()?;
    m.add_class::<GCPConfig>()?;
    m.add_class::<GiteaConfig>()?;
    m.add_class::<RangerConfig>()?;
    m.add_class::<PrestoConfig>()?;
    m.add_class::<PostgresConfig>()?;
    m.add_class::<LINZAPIConfig>()?;
    m.add_class::<MinioConfig>()?;
    m.add_class::<PDALConfig>()?;
    Ok(())
}
