from aorist import aorist, DownloadDataFromRemoteWebLocation

programs = {}

@aorist(
    programs,
    DownloadDataFromRemoteWebLocation,
    entrypoint="download_data_from_remote_web_location",
    args={
        "dataset_name": lambda lng: lng.data_set.name,
        "table_name": lambda lng: lng.static_data_table.name,
        "src_url": lambda lng: lng.web_location.address,
        "tmp_dir": lambda lng: lng.replication_storage_setup.tmp_dir,
        "dest_file_name": lambda lng: lng.{
  format!(
      "{}.{}",
      static_data_table.name,
      match static_data_table.setup {
           StorageSetup::ReplicationStorageSetup(ref x) => x.get_download_extension(),
           _ => panic!("Storage setup must be ReplicationStorageSetup"),
      }
  )
}
,
    },
)
def recipe(dataset_name, table_name, src_url, tmp_dir, dest_file_name):
    import os
    import urllib.request
    
    def download_data_from_remote_web_location(src_url, tmp_dir, dest_file_name, table_name, dataset_name):
        os.makedirs(tmp_dir + '/' + dataset_name + '/' + table_name, exist_ok=True)
        urllib.request.urlretrieve(src_url, os.path.join(tmp_dir, dataset_name, table_name, dest_file_name))
    
    