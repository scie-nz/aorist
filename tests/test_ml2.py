from aorist import *
from aorist_recipes import programs
from scienz import build_subreddit_assets, subreddit_schema, subreddit_datum

local = SQLiteStorage(
    location=SQLiteLocation(file_name='subreddits.sqlite'),
    layout=TabularLayout(StaticTabularLayout()),
)
subreddits = DataSet(
    name="subreddits",
    description="""
    r/subreddits
    """,
    source_path=__file__,
    datum_templates=[DatumTemplate(subreddit_datum)],
    assets=build_subreddit_assets(["probprog", "mlops"]),
    access_policies=[],
)
subreddits = subreddits.replicate_to_local(
    Storage(local), "/tmp/probprog", Encoding(CSVEncoding())
)
embedding = FasttextEmbedding(
    name="embedding",
    comment="Fasttext embedding of size 16",
    schema=DataSchema(FasttextEmbeddingSchema(
        dim=16,
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
result = dag(universe, ["UploadFasttextToSQLite"], 
             "python", programs)
with open('generated_script_ml2.py', 'w') as f:
    f.write(result)
