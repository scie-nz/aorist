from aorist import aorist, UploadDataToS3

programs = {}

@aorist(
    programs,
    UploadDataToS3,
    entrypoint="upload_to_s3",
    args={
        "access_key": lambda universe: universe.endpoints.aws.access_key_id,
        "secret_key": lambda universe: universe.endpoints.aws.access_key_secret,
        "bucket": lambda s3_location: s3_location.bucket,
        "schema": lambda data_set: data_set.name,
        "tablename": lambda static_data_table: "{}_csv".format(static_data_table.name),
        "tmp_dir": lambda replication_storage_setup: replication_storage_setup.tmp_dir,
        "source_file": lambda data_set, static_data_table: "{}/{}/data.csv".format(data_set.name, static_data_table.name),
    },
)
def recipe(access_key, secret_key, bucket, schema, tablename, tmp_dir, source_file):
    
    import logging
    import boto3
    from botocore.exceptions import ClientError
    
    def upload_to_s3(access_key, secret_key, bucket, schema, tablename, tmp_dir, source_file):
        client = boto3.client(
            's3',
            aws_access_key_id=access_key,
            aws_secret_access_key=secret_key,
        )
        source_path = tmp_dir + "/" + source_file
        dest_path = schema + '/' + tablename + '/data.csv'
        response = client.upload_file(source_path, bucket, dest_path)
    
    
