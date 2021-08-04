import inspect
import copy
from aorist import *
from aorist_recipes import programs
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
    encoding=Encoding(NewlineDelimitedJSONEncoding()),
    layout=TabularLayout(StaticTabularLayout()),
)
"""
Declaring where our subreddits live, i.e. in PushShift
"""
subreddits = ['france']#, 'newzealand']
tabular_schema = default_tabular_schema(
    subreddit_datum, subreddit_datum.name, attributes
)
assets = {x: StaticDataTable(
    name=x,
    schema=DataSchema(tabular_schema),
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

embedding = FasttextEmbedding(
    name="embedding",
    comment="Fasttext embedding of size 128",
    schema=DataSchema(FasttextEmbeddingSchema(
        dim=128,
        source_schema=tabular_schema,
        text_attribute_name="selftext",
    )),
    setup=StorageSetup(LocalStorageSetup(
        Storage(local),
        '/tmp/fasttext_embedding',
    )),
    source_assets=[Asset(v) for v in assets.values()],
)
assets = {
    k: Asset(v) for (k, v) in assets.items()
}
assets['embedding'] = Asset(embedding)
"""
Creating the dataset
"""
subreddits = DataSet(
    name="subreddits",
    description="""
    A selection of small region-based Subreddits to demonstrate
    collecting Reddit data via [Pushshift](https://pushshift.io/).
    """,
    source_path=__file__,
    datum_templates=[DatumTemplate(subreddit_datum)],
    assets=assets,
    access_policies=[],
)
"""
Dataset will be replicated.
"""
subreddits = subreddits.replicate_to_local(
    Storage(local), tmp_dir, Encoding(CSVEncoding())
)
universe = Universe(
    name="my_cluster",
    datasets=[subreddits],
    endpoints=endpoints,
)
universe.compute_uuids()

result = dag(
    universe,
    ["UploadFasttextToMinio"],
    "airflow",
    programs,
    dialect_preferences=[
        R(),
        Python([]),
        Bash(),
        Presto(),
    ],
)
print(result)
