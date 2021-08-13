from aorist import *
from aorist_recipes import programs
from scienz import (
    us_subreddits, subreddit_schema
)

local = SQLiteStorage(
    location=SQLiteLocation(file_name='subreddits.sqlite'),
    layout=TabularLayout(StaticTabularLayout()),
)
subreddits = us_subreddits.replicate_to_local(
    Storage(local), "/tmp/us_subreddits", Encoding(CSVEncoding())
)
embedding = FasttextEmbedding(
    name="embedding",
    comment="Fasttext embedding of size 128",
    schema=DataSchema(FasttextEmbeddingSchema(
        dim=128,
        source_schema=subreddit_schema,
        text_attribute_name="selftext",
    )),
    setup=StorageSetup(LocalStorageSetup(
        Storage(local),
        '/tmp/fasttext_embedding',
    )),
    source_assets=list(subreddits.assets.values()),
)
subreddits.add_asset('embedding', Asset(embedding))
universe = Universe(
    name="my_cluster",
    datasets=[subreddits],
    endpoints=EndpointConfig(),
    compliance=None,
)

universe.compute_uuids()
result = dag(
    universe,
    ["UploadFasttextToSQLite"],
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
