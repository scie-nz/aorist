import copy
from aorist import *

"""
Defining endpoints.
"""
alluxio_config = AlluxioConfig(
    server="alluxio-server",
    server_cli="alluxio-server",
    rpc_port=19998,
    api_port=39999,
    directory="data",
)
aws_config = AWSConfig(
    aws_key_env_var="[AWS_ACCESS_KEY]",
    aws_key_secret_env_var="[AWS_ACCESS_KEY_SECRET]",
)
ranger_config = RangerConfig(
    server="localhost", user="admin", password="G0powerRangers", port=8088,
)
presto_config = PrestoConfig(server="trino-coordinator-headless", user="bogdan", http_port=8080)
gitea_config = GiteaConfig(token="2b44b07e042ee9fe374e3eeebd2c9098468b5774", server="localhost", port=8079)
minio_config = MinioConfig(
    server="minio",
    port=9000,
    bucket="minio-test-bucket",
    access_key="cppBrbSkEg5Vet6Mb0D4",
    secret_key="eRtRoywXqKBj0yHDyIaYb0c1Xnr5A3mCGsiT67Y1",
)
postgres_config = PostgresConfig(
    server='172.31.81.125',
    port=5433,
    username='airflow',
    password='RZpFdxPealPBKYCNfZqnujixSdqkjXVV',
)
endpoints = EndpointConfig(
    alluxio=alluxio_config,
    ranger=ranger_config,
    presto=presto_config,
    gitea=gitea_config,
    minio=minio_config,
    postgres=postgres_config,
    aws=aws_config,
    gcp=GCPConfig(
        project_name='social-norms',
        data_location='US',
        service_account_file='/home/bogdan/.gcloud/climate-change-289705-1592601e084f.json',
        use_default_credentials=False,
    ),
)
