mod hive_table_storage;
mod remote_website_storage;
mod storage;

pub use hive_table_storage::{InnerHiveTableStorage, HiveTableStorage};
pub use remote_website_storage::{InnerRemoteStorage, RemoteStorage};
pub use storage::{InnerStorage, Storage};
