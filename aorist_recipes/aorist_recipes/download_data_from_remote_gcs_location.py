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
            context.capture(
                "is_json",
                dumps(replication_storage_setup.source.encoding.newline_delimited_json_encoding is not None),
            ),
            context,
        ),
        "_delimiter": lambda replication_storage_setup, context: (
            context.capture(
                "delimiter",
                dumps(
                    "\t" if replication_storage_setup.source.encoding.tsv_encoding is not None
                    else "," if replication_storage_setup.source.encoding.csv_encoding is not None 
                    else None
                ),
            ),
            context,
        ),
        "file_to_replicate": lambda static_data_table, context: (context.capture(
            "file_to_replicate",
            ("{file_name}.{extension}").format(
                file_name=static_data_table.name,
                extension=static_data_table.setup.replication_storage_setup.download_extension,
            )
        ), context)
    },
)
def recipe(bucket_name, blob_name, tmp_dir, file_to_replicate, credentials, _is_json, _delimiter):
    from google.cloud import storage
    import os
    def download_blob_to_file(bucket_name, blob_name, tmp_dir, file_to_replicate, credentials, _is_json, _delimiter):
      if credentials != "":
          client = storage.Client.from_service_account_json(credentials)
      else:
          client = storage.Client()

      bucket = client.bucket(bucket_name)
      blob = bucket.blob(blob_name)
      if not os.path.exists(tmp_dir):
          os.makedirs(tmp_dir)
      dest = "%s/%s" % (tmp_dir, file_name)
      blob.download_to_filename(dest)
