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
    Storage(local), "/tmp/subreddits", Encoding(CSVEncoding())
)
text_source_schema = TextCorpusSchema(
    source=TextCorpusSource(subreddit_schema),
    text_attribute_name="title",
)

source_assets = list(subreddits.assets.values())

embedding = FasttextEmbedding(
    name="embedding",
    comment="Fasttext embedding of size 128",
    schema=DataSchema(LanguageAssetSchema(FasttextEmbeddingSchema(
        dim=16,
        source_schema=text_source_schema
    ))),
    setup=StorageSetup(LocalStorageSetup(
        Storage(local),
        '/tmp/subreddits',
    )),
    source_assets=source_assets,
)
subreddits.add_asset('embedding', Asset(LanguageAsset(embedding)))

named_entities = NamedEntities(
    name="named_entities",
    comment="Spacy Named Entities",
    schema=DataSchema(LanguageAssetSchema(NamedEntitySchema(SpacyNamedEntitySchema(
        spacy_model_name="en_core_web_sm",
        source_schema=text_source_schema,
    )))),
    setup=StorageSetup(LocalStorageSetup(
        Storage(local),
        '/tmp/subreddits',
    )),
    source_assets=source_assets,
)
subreddits.add_asset('named_entities', Asset(LanguageAsset(named_entities)))
    
universe = Universe(
    name="my_cluster",
    datasets=[subreddits],
    endpoints=EndpointConfig(),
    compliance=None,
)
result = dag(universe, ["ExtractNamedEntitiesUsingSpacy", "TrainFasttextModel"], 
             "airflow", programs)
with open('generated_script_ml.py', 'w') as f:
    f.write(result)
