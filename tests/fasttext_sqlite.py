from aorist import *
from aorist_recipes import programs
from scienz import (
    probprog, subreddit_schema
)

local = SQLiteStorage(
    location=SQLiteLocation(file_name='subreddits.sqlite'),
    layout=TabularLayout(StaticTabularLayout()),
)
subreddits = probprog.replicate_to_local(
    Storage(local), "/tmp/probprog", Encoding(CSVEncoding())
)
text_source_schema = TextCorpusSchema(
    source=TextCorpusSource(subreddit_schema),
    text_attribute_name="selftext",
)
embedding = FasttextEmbedding(
    name="embedding",
    comment="Fasttext embedding of size 128",
    schema=DataSchema(FasttextEmbeddingSchema(
        dim=16,
        source_schema=text_source_schema
    )),
    setup=StorageSetup(LocalStorageSetup(
        Storage(local),
        '/tmp/fasttext_embedding',
    )),
    source_assets=list(subreddits.assets.values()),
)
subreddits.add_asset('embedding', Asset(LanguageAsset(embedding)))
named_entities = NamedEntities(
    name="named_entities",
    comment="Spacy Named Entities",
    schema=DataSchema(NamedEntitySchema(SpacyNamedEntitySchema(
        spacy_model_name="en_core_web_sm",
        source_schema=text_source_schema,
    ))),
    setup=StorageSetup(LocalStorageSetup(
        Storage(local),
        '/tmp/named_entities',
    )),
    source_assets=list(subreddits.assets.values()),
)
subreddits.add_asset('named_entities', Asset(LanguageAsset(named_entities)))
    
universe = Universe(
    name="my_cluster",
    datasets=[subreddits],
    endpoints=EndpointConfig(),
    compliance=None,
)
result = dag(universe, ["TextCorpusDataFromSQLite"], 
             "python", programs)
with open('generated_script_ml.py', 'w') as f:
    f.write(result)
