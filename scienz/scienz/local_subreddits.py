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
tabular_schema = default_tabular_schema(
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

assets = {x: Asset(StaticDataTable(
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
    )) for x in local_subreddits}

us_assets = {x: Asset(StaticDataTable(
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
    )) for x in us_subreddits}

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
