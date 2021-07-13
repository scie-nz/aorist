/***
@aorist_presto(
    programs,
    HiveDirectoriesCreated,
    args={
        "presto_schema": lambda data_set, context: (
            context.capture("schema", data_set.name),
            context,
        ),
        "location": lambda hive_table_storage, universe, data_set, context: (
        context.capture("location",
            "WITH (location='alluxio://{server}:{port}/{directory}/{dataset}/{path}')".format(
                server=universe.endpoints.alluxio.server,
                port=universe.endpoints.alluxio.rpc_port,
                directory=universe.endpoints.alluxio.directory,
                dataset=data_set.name,
                path=hive_table_storage.location.alluxio_location,
        ) if hive_table_storage.location.alluxio_location is not None else (
            "WITH (location='s3a://{bucket}/{dataset}/')".format(
                bucket=universe.endpoints.minio.bucket,
                dataset=data_set.name,
            )
        ) if hive_table_storage.location.minio_location is not None else (
            "WITH (location='s3a://{bucket}/{dataset}/')".format(
                bucket=universe.endpoints.s3.bucket,
                dataset=data_set.name,
            )
        ) if hive_table_storage.location.s3_location is not None else (
            panic("Only Alluxio, MinIO or S3 locations supported.")
        )), context)
    },
)
***/
CREATE SCHEMA IF NOT EXISTS {presto_schema} {location}
