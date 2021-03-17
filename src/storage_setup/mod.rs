mod computed_from_local_data;
mod replication_storage_setup;
mod remote_storage_setup;
mod storage_setup;

pub use computed_from_local_data::{ComputedFromLocalData, InnerComputedFromLocalData};
pub use replication_storage_setup::{InnerReplicationStorageSetup, ReplicationStorageSetup};
pub use remote_storage_setup::{InnerRemoteStorageSetup, RemoteStorageSetup};
pub use storage_setup::{InnerStorageSetup, StorageSetup};
