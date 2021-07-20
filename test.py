import inspect
import copy
from aorist import *
from recipes import programs
from common import endpoints

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
subreddits = ['france', 'newzealand']
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
    "jupyter",
    programs,
    dialect_preferences=[
        R(),
        Python([]),
        Bash(),
        Presto(),
    ],
)
print(result)
