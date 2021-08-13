from aorist import aorist, UploadFasttextToMinio

programs = {}

@aorist(
    programs,
    UploadFasttextToMinio,
    entrypoint="upload_fasttext_to_minio",
    args={
        "hostname": lambda universe: universe.endpoints.minio.server,
        "port": lambda universe: "%s" % universe.endpoints.minio.port,
        "access_key": lambda universe: universe.endpoints.minio.access_key,
        "secret_key": lambda universe: universe.endpoints.minio.secret_key,
        "bucket": lambda universe: universe.endpoints.minio.bucket,
        "schema": lambda data_set: data_set.name,
        "table_name": lambda fasttext_embedding: fasttext_embedding.name,
        "tmp_dir": lambda fasttext_embedding: fasttext_embedding.setup.local_storage_setup.tmp_dir
    },
)
def recipe(hostname, port, access_key, secret_key, bucket, schema, table_name, tmp_dir):
    from minio import Minio
    def upload_fasttext_to_minio(
        hostname, port, access_key, secret_key, bucket, schema, table_name, tmp_dir,
    ):
        client = Minio(
            "%s:%s" % (hostname, port),
            access_key=access_key,
            secret_key=secret_key,
            secure=False,
        )
        assert client.bucket_exists(bucket)
        dest_path = schema + '/' + tablename + '/data.csv'
        source_path = tmp_dir + "/words.txt"
        client.fput_object(bucket, dest_path, source_path)
        print("Successfully uploaded %s to %s" % (source_path, dest_path))
