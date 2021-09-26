from aorist import aorist
from aorist import DownloadDataFromRemoteGCSLocation
from json import dumps

programs = {}

@aorist(
    programs,
    DownloadDataFromRemoteGCSLocation,
    entrypoint="download_blob_to_file",
    args={
        "bucket_name": lambda gcs_location: gcs_location.bucket,
        "blob_name": lambda gcs_location: gcs_location.blob,
        "credentials": lambda universe: universe.endpoints.gcp.service_account_file,
        "tmp_dir": lambda replication_storage_setup, context: (
            context.capture(
                "downloaded_tmp_dir",
                replication_storage_setup.tmp_dir,
            ),
            context,
        ),
        "_is_json": lambda replication_storage_setup, context: (
            context.capture_bool(
                "is_json",
                replication_storage_setup.source.encoding.newline_delimited_json_encoding is not None,
            ),
            context,
        ),
        "_delimiter": lambda replication_storage_setup, context: (
            context.capture(
                "delimiter",
                "\t" if replication_storage_setup.source.encoding.tsv_encoding is not None
                else "," if replication_storage_setup.source.encoding.csv_encoding is not None 
                else None
            ),
            context,
        ),
        "_header_num_lines": lambda replication_storage_setup, context: (
            context.capture_int(
                "header_num_lines",
                replication_storage_setup.source.encoding.header.num_lines 
                if replication_storage_setup.source.encoding.header is not None 
                else 0,
            ),
            context,
        ),
        "dest": lambda replication_storage_setup, static_data_table, context: (context.capture(
            "file_to_replicate",
            ("{tmp_dir}/{file_name}.{extension}").format(
                tmp_dir=replication_storage_setup.tmp_dir,
                file_name=static_data_table.name,
                extension=static_data_table.setup.replication_storage_setup.download_extension,
            )
        ), context)
    },
)
def recipe(bucket_name, blob_name, tmp_dir, dest, credentials):
    from google.cloud import storage
    import os
    def download_blob_to_file(bucket_name, blob_name, tmp_dir, dest, credentials):
      if credentials != "":
          client = storage.Client.from_service_account_json(credentials)
      else:
          client = storage.Client()

      bucket = client.bucket(bucket_name)
      blob = bucket.blob(blob_name)
      if not os.path.exists(tmp_dir):
          os.makedirs(tmp_dir)
      if not os.path.exists(dest):
          blob.download_to_filename(dest)
      print("Downloaded file: %s" % dest)
