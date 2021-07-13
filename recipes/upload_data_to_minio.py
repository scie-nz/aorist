from aorist import aorist, UploadDataToMinio

programs = {}

@aorist(
    programs,
    UploadDataToMinio,
    entrypoint="upload_to_minio",
    args={
        "hostname": lambda universe: universe.endpoints.minio.server,
        "port": lambda universe: "%s" % universe.endpoints.minio.port,
        "access_key": lambda universe: universe.endpoints.minio.access_key,
        "secret_key": lambda universe: universe.endpoints.minio.secret_key,
        "bucket": lambda universe: universe.endpoints.minio.bucket,
        "schema": lambda data_set, context: (context.get("schema"), context),
        "tablename": lambda static_data_table: "%s_csv" % static_data_table.name,
        "tmp_dir": lambda replication_storage_setup: replication_storage_setup.tmp_dir,
        "source_file": lambda static_data_table: "%s.csv" % static_data_table.name,
    }
)
def recipe(hostname, port, access_key, secret_key, bucket, schema, tablename, tmp_dir, source_file):
    from minio import Minio
    def upload_to_minio(hostname, port, access_key, secret_key, bucket, schema, tablename, tmp_dir, source_file):
        client = Minio(
            "%s:%s" % (hostname, port),
            access_key=access_key,
            secret_key=secret_key,
            secure=False,
        )
        assert client.bucket_exists(bucket)
        dest_path = schema + '/' + tablename + '/data.csv'
        source_path = tmp_dir + "/" + source_file
        client.fput_object(bucket, dest_path, source_path)
        print("Successfully uploaded %s to %s" % (source_path, dest_path))


