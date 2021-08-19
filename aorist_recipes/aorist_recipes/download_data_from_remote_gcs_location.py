from aorist import aorist
from aorist import DownloadDataFromRemoteGCSLocation

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
        "dest_file_name": lambda static_data_table: (
            "{file_name}.{extension}"
        ).format(
            file_name=static_data_table.name,
            extension=static_data_table.setup.replication_storage_setup.download_extension,
        )
    },
)
def recipe(bucket_name, blob_name, tmp_dir, dest_file_name, credentials):
    from google.cloud import storage
    import os
    def download_blob_to_file(bucket_name, blob_name, tmp_dir, dest_file_name, credentials):
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
