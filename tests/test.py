from aorist import *
from aorist_recipes import programs
from scienz import (
    us_subreddits, subreddit_schema
)
from common import endpoints

local = HiveTableStorage(
    location=HiveLocation(MinioLocation(name='reddit')),
    encoding=Encoding(NewlineDelimitedJSONEncoding()),
    layout=TabularLayout(StaticTabularLayout()),
)
subreddits = us_subreddits.replicate_to_local(
    Storage(local), "/tmp/us_subreddits", Encoding(CSVEncoding())
)
text_template = DatumTemplate(Text(name="corpus"))
text_corpus_schema = TextCorpusSchema(
    sources=[x.static_data_table for x in subreddits.assets.values()],
    datum_template=text_template,
    text_attribute_name="TEXT",
)
text_corpus = TextCorpus(
    name="text_corpus",
    comment="Subreddits",
    schema=DataSchema(LanguageAssetSchema(text_corpus_schema)),
    setup=StorageSetup(LocalStorageSetup(
        Storage(local),
        '/tmp/subreddits',
    )),
)
subreddits.add_asset(Asset(LanguageAsset(text_corpus)))
fasttext_attributes = [
    Attribute(KeyStringIdentifier("word_id")),
    Attribute(FreeText("word")),
    Attribute(FreeText("embedding")),
]
fasttext_datum = RowStruct(
    name="fasttext",
    attributes=fasttext_attributes,
)
embedding = LanguageAsset(FasttextEmbedding(
    name="embedding",
    comment="Fasttext embedding of size 128",
    schema=DataSchema(LanguageAssetSchema(FasttextEmbeddingSchema(
        dim=128,
        source=text_corpus,
        datum_template=DatumTemplate(fasttext_datum),
    ))),
    setup=StorageSetup(LocalStorageSetup(
        Storage(local),
        '/tmp/subreddits',
    )),
))
subreddits.add_asset(Asset(embedding))
universe = Universe(
    name="my_cluster",
    datasets=[subreddits],
    endpoints=endpoints,
    compliance=None,
)

universe.compute_uuids()
result = dag(
    universe,
    ["UploadFasttextToMinio"],
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
