from aorist.target.debug.libaorist import S3Location
from aorist_constraint.target.debug.libaorist_constraint import UploadDataToS3

program = UploadDataToS3.register_program("""
    import foo
""")
print(program)

def aorist(programs, constraint, *args, **kwargs):
    def inner(func):
        programs[constraint] = constraint.register_program(str(func))
    return inner

programs = {}

@aorist(
    programs,
    UploadDataToS3,
    entrypoint="upload_to_s3",
    args={
        "access_key": lambda ancestry : ancestry.universe.endpoints.access_key_id, 
        "secret_key": lambda ancestry : ancestry.universe.endpoints.access_key_secret, 
        "table_name": lambda ancestry : ancestry.data_set.name + ".csv",
        "bucket": lambda ancestry: ancestry.s3location.bucket,
        "schema": lambda ancestry: ancestry.data_set.name,
        "tmp_dir": lambda ancestry: ancestry.replication_storage_setup.tmp_dir,
        "source_file": lambda ancestry: "%s_%s" % (
            ancestry.data_set.name,
            ancestry.static_data_table.name,
        )
    }
)
def recipe():
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

