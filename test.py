import inspect
import copy
from aorist import *
from recipes import programs

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

"""
Defining schema
"""
attributes = [
    Attribute(KeyStringIdentifier("id")),
    Attribute(StringIdentifier("author")),
    Attribute(StringIdentifier("subreddit")),
    Attribute(POSIXTimestamp("created_utc")),
    Attribute(FreeText("title")),
    Attribute(FreeText("selftext", nullable=True)),
]
"""
A row in our table is a struct.
"""
subreddit_datum = RowStruct(
    name="subreddit",
    attributes=attributes,
)
tmp_dir = "tmp/subreddits"
"""
Data will be replicated to Hive
"""
local = HiveTableStorage(
    location=HiveLocation(MinioLocation(name='reddit')),
    encoding=Encoding(CSVEncoding()),
    layout=TabularLayout(StaticTabularLayout()),
)
"""
Declaring where our subreddits live, i.e. in PushShift
"""
subreddits = ['france']
assets = {x: StaticDataTable(
    name=x,
    schema=default_tabular_schema(subreddit_datum, x, attributes),
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
    ) for x in subreddits}

"""
Creating the dataset
"""
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
"""
Dataset will be replicated.
"""
subreddits = subreddits.replicate_to_local(Storage(local), tmp_dir, Encoding(CSVEncoding()))
universe = Universe(
    name="my_cluster",
    datasets=[subreddits],
    endpoints=endpoints,
)
universe.compute_uuids()

result = dag(
    universe,
    ["UploadDataToMinio", "JSONTableSchemasCreated"],
    "python",
    programs,
    dialect_preferences=[
        R(),
        Python([]),
        Bash(),
        Presto(),
    ],
)
print(result)
