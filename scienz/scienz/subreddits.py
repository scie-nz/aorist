import json
import os
from aorist import (
    Attribute,
    KeyStringIdentifier,
    StringIdentifier,
    POSIXTimestamp,
    FreeText,
    default_tabular_schema,
    RowStruct,
    StaticDataTable,
    DataSchema,
    StorageSetup,
    RemoteStorageSetup,
    Storage,
    RemoteStorage,
    RemoteLocation,
    PushshiftAPILocation,
    APIOrFileLayout,
    APILayout,
    PushshiftSubredditPostsAPILayout,
    Encoding,
    NewlineDelimitedJSONEncoding,
    DataSet,
    DatumTemplate,
    Asset,
)
from aorist import *

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
subreddit_schema = default_tabular_schema(
    DatumTemplate(subreddit_datum), attributes
)

local_subreddits = json.load(
    open(os.path.join(os.path.dirname(__file__), 'subreddits.json'))
)
us_subreddits = ["newyork", "sanfrancisco", "chicago",
                 "miami", "seattle", "neworleans", "atlanta",
                 "boston", "baltimore", "philadelphia", "dc",
                 "dallas", "houston", "sanantonio", "denver",
                 "losangeles", "portland", "cleveland", "columbus",
                 "charlotte", "detroit", "pittsburgh", "stlouis",
                 "kansascity"]

def build_assets(subreddit_names):
    return {x: Asset(StaticDataTable(
        name=x,
        schema=DataSchema(subreddit_schema),
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
        )) for x in subreddit_names}

assets = build_assets(local_subreddits)
us_assets = build_assets(us_subreddits)
nz_assets = build_assets(["wellington", "auckland", "chch", "thetron", "dunedin", "tauranga",
                          "gisborne", "napier", "nelson", "palmy", "queenstown", "newplymouth"])
datascience = build_assets(["datascience"])
place_based_subreddits = DataSet(
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
us_subreddits = DataSet(
    name="us_subreddits",
    description="""
    A selection of small region-based Subreddits in the US to demonstrate
    collecting Reddit data via [Pushshift](https://pushshift.io/).
    """,
    source_path=__file__,
    datum_templates=[DatumTemplate(subreddit_datum)],
    assets=us_assets,
    access_policies=[],
)
nz_subreddits = DataSet(
    name="nz_subreddits",
    description="""
    A selection of small region-based Subreddits in New Zealand to demonstrate
    collecting Reddit data via [Pushshift](https://pushshift.io/).
    """,
    source_path=__file__,
    datum_templates=[DatumTemplate(subreddit_datum)],
    assets=nz_assets,
    access_policies=[],
)
datascience = DataSet(
    name="datascience",
    description="""
    r/datascience
    """,
    source_path=__file__,
    datum_templates=[DatumTemplate(subreddit_datum)],
    assets=datascience,
    access_policies=[],
)
datamining = DataSet(
    name="datamining",
    description="""
    r/datamining
    """,
    source_path=__file__,
    datum_templates=[DatumTemplate(subreddit_datum)],
    assets=build_assets(["datamining"]),
    access_policies=[],
)
probprog = DataSet(
    name="probprog",
    description="""
    r/probprog
    """,
    source_path=__file__,
    datum_templates=[DatumTemplate(subreddit_datum)],
    assets=build_assets(["probprog"]),
    access_policies=[],
)

### TODO: move elsewhere
fasttext_attributes = [
    Attribute(KeyStringIdentifier("word_id")),
    Attribute(FreeText("word")),
    Attribute(FreeText("embedding")),
]
fasttext_datum = RowStruct(
    name="fasttext",
    attributes=fasttext_attributes,
)

spacy_ner_attributes = [
    Attribute(Int64Identifier("line_id")),
    Attribute(KeyStringIdentifier("entity_id")),
    Attribute(FreeText("entity_text")),
    Attribute(FreeText("text")),
    Attribute(Count("start")),
    Attribute(Count("end")),
    Attribute(Factor("label")),
]
spacy_ner_datum = RowStruct(
    name="spacy_ner",
    attributes=spacy_ner_attributes,
)

local = SQLiteStorage(
    location=SQLiteLocation(file_name='probprog.sqlite'),
    layout=TabularLayout(StaticTabularLayout()),
)
probprog = probprog.replicate_to_local(
    Storage(local), "/tmp/probprog", Encoding(CSVEncoding())
)
source_assets = list(probprog.assets.values())
text_source_schema = TextCorpusSchema(
    sources=[x.static_data_table for x in source_assets],
    text_attribute_name="title",
)
embedding = FasttextEmbedding(
    name="embedding",
    comment="Fasttext embedding of size 128",
    schema=DataSchema(LanguageAssetSchema(FasttextEmbeddingSchema(
        dim=16,
        source_schema=text_source_schema,
        datum_template=DatumTemplate(fasttext_datum)
    ))),
    setup=StorageSetup(LocalStorageSetup(
        Storage(local),
        '/tmp/probprog',
    )),
    source_assets=source_assets,
)
probprog.add_asset(Asset(LanguageAsset(embedding)))
named_entities = NamedEntities(
    name="named_entities",
    comment="Spacy Named Entities",
    schema=DataSchema(LanguageAssetSchema(NamedEntitySchema(SpacyNamedEntitySchema(
        spacy_model_name="en_core_web_sm",
        source_schema=text_source_schema,
        datum_template=DatumTemplate(spacy_ner_datum),
    )))),
    setup=StorageSetup(LocalStorageSetup(
        Storage(local),
        '/tmp/probprog',
    )),
    source_assets=source_assets,
)
probprog.add_asset(Asset(LanguageAsset(named_entities)))
