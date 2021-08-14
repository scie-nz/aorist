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
    subreddit_datum, subreddit_datum.name, attributes
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
nz_assets = build_assets(["marton"])
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
