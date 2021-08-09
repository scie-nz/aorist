from aorist import aorist, DownloadDataFromRemoteWebLocation

programs = {}

@aorist(
    programs,
    DownloadDataFromRemoteWebLocation,
    entrypoint="download_data_from_remote_web_location",
    args={
        "dataset_name": lambda data_set: data_set.name,
        "table_name": lambda static_data_table: static_data_table.name,
        "src_url": lambda web_location: web_location.address,
        "tmp_dir": lambda replication_storage_setup, context: (
            context.capture(
                "downloaded_tmp_dir",
                replication_storage_setup.tmp_dir,
            ),
            context,
        ),
        "dest_file_name": lambda static_data_table: (
            "{file_name}.{extension}"
        ).format(
            file_name=static_data_table.name,
            extension=static_data_table.setup.replication_storage_setup.download_extension,
        )
    },
)
def recipe(dataset_name, table_name, src_url, tmp_dir, dest_file_name):
    import os
    import urllib.request
    
    def download_data_from_remote_web_location(src_url, tmp_dir, dest_file_name, table_name, dataset_name):
        os.makedirs(tmp_dir + '/' + dataset_name + '/' + table_name, exist_ok=True)
        urllib.request.urlretrieve(src_url, os.path.join(tmp_dir, dataset_name, table_name, dest_file_name))
    
    
