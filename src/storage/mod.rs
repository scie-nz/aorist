mod hive_table_storage;
mod local_file_storage;
mod remote_website_storage;
mod storage;

pub use hive_table_storage::{HiveTableStorage, InnerHiveTableStorage};
pub use local_file_storage::*;
pub use remote_website_storage::{InnerRemoteStorage, RemoteStorage};
pub use storage::{InnerStorage, Storage};
