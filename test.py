import inspect
from aorist.target.debug.libaorist import *

def default_tabular_schema(datum, template_name):
    return DataSchema(TabularSchema(
        datumTemplateName=template_name,
        attributes=[a.name for a in attributes],
    ))

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
    access_key_id="[AWS_ACCESS_KEY]",
    access_key_secret="[AWS_ACCESS_KEY_SECRET]",
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
attributes = [
    Attribute(KeyStringIdentifier("id")),
    Attribute(StringIdentifier("author")),
    Attribute(StringIdentifier("subreddit")),
    Attribute(POSIXTimestamp("created_utc")),
    Attribute(FreeText("title")),
    Attribute(FreeText("selftext", nullable=True)),
]
subreddit_datum = RowStruct(
    name="subreddit",
    attributes=attributes,
)
print(attributes)
tmp_dir = "tmp/subreddits"
local = BigQueryStorage(
    location=BigQueryLocation(),
    layout=TabularLayout(StaticTabularLayout()),
)
subreddits = ['wairarapa', 'marton', 'marlborough']
assets = {x: StaticDataTable(
    name=x,
    schema=default_tabular_schema(subreddit_datum, x),
    setup=StorageSetup(RemoteStorageSetup(
        remote=Storage(RemoteStorage(
            location=RemoteLocation(
                PushshiftAPILocation(
                    subreddit=x
                )
            ),
            layout=APIOrFileLayout(
                APILayout(
                    PushshiftSubredditPostsAPILayout()
                ),
            ),
            encoding=Encoding(
                NewlineDelimitedJSONEncoding()
            ),
        )),
    )),
    tag=x,
    ) for x in subreddits[:1]}

subreddits = DataSet(
    name="subreddits",
    description="A selection of small region-based Subreddits to demonstrate collecting Reddit data via [Pushshift](https://pushshift.io/).",
    source_path=__file__,
    datum_templates=[DatumTemplate(subreddit_datum)],
    assets={
        k: Asset(v) for (k, v) in assets.items()
    },
    access_policies=[],
)
subreddits.replicate_to_local(Storage(local), tmp_dir, Encoding(CSVEncoding()))
universe = Universe(
    name="my_cluster",
    datasets=[subreddits],
    endpoints=endpoints,
)
print(universe)


pushshift = PushshiftAPILocation(
    subreddit="newzealand",
)
print(pushshift)
print(pushshift.subreddit)
pushshift.subreddit = "wellington"
print(pushshift.subreddit)
location = RemoteLocation(pushshift)

print(location)

from aorist_constraint.target.debug.libaorist_constraint import (
    DownloadDataFromRemotePushshiftAPILocationToNewlineDelimitedJSON
)

def to_str(source):
    funcString = "\n".join([str(x) for x in inspect.getsourcelines(source)])
    return funcString


def aorist(programs, constraint, entrypoint, args):
    args_str = {k : to_str(v) for k, v in args.items()}
    def inner(func):
        programs[constraint] = constraint.register_program(to_str(func), entrypoint, args_str)
    return inner

programs = {}

@aorist(
    programs,
    DownloadDataFromRemotePushshiftAPILocationToNewlineDelimitedJSON,
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

print(ConceptAncestry)
print(programs)
