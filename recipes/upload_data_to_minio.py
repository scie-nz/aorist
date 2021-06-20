from aorist import aorist, UploadDataToMinio

programs = {}

@aorist(
    programs,
    UploadDataToMinio,
    entrypoint="upload_to_minio",
    args={
        "hostname": lambda lng: lng.universe.endpoints.minio.server,
        "port": lambda lng: lng.universe.endpoints.minio.port,
        "access_key": lambda lng: lng.universe.endpoints.minio.access_key,
        "secret_key": lambda lng: lng.universe.endpoints.minio.secret_key,
        "bucket": lambda lng: lng.universe.endpoints.minio.bucket,
        "schema": lambda lng: lng.data_set.name,
        "tablename": lambda lng: lng.format!("{}_csv", static_data_table.name),
        "tmp_dir": lambda lng: lng.replication_storage_setup.tmp_dir,
        "source_file": lambda lng: lng.format!("{}.csv", static_data_table.name),
    },
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
    
    