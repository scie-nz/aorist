
from aorist import (
    RowStruct,
    MinioLocation,
    WebLocation,
    StaticTabularLayout,
    ORCEncoding,
    CSVEncoding,
    SingleFileLayout,
    RemoteStorage,
    HiveTableStorage,
    RemoteStorageSetup,
    StaticDataTable,
    DataSet,
    default_tabular_schema,
    attr_list,
    PushshiftAPILocation,
    PushshiftSubredditPostsAPILayout,
    NewlineDelimitedJSONEncoding,
)
from aorist import attributes as attr

attributes = attr_list([
    attr.KeyStringIdentifier("id"),
    attr.StringIdentifier("author"),
    attr.StringIdentifier("subreddit"),
    attr.POSIXTimestamp("created_utc"),
    attr.FreeText("title"),
    attr.FreeText("selftext", nullable=True),
])
subreddit_datum = RowStruct(
    name="subreddit",
    attributes=attributes,
)
subreddits = ['wairarapa', 'marton', 'marlborough']
assets = {x: StaticDataTable(
    name=x,
    schema=default_tabular_schema(subreddit_datum),
    setup=RemoteStorageSetup(
        remote=RemoteStorage(
            location=PushshiftAPILocation(subreddit=x),
            layout=PushshiftSubredditPostsAPILayout(),
            encoding=NewlineDelimitedJSONEncoding(),
        ),
    ),
    tag=x,
    ) for x in subreddits[:1]}

subreddits = DataSet(
    name="subreddits",
    description="A selection of small region-based Subreddits to demonstrate collecting Reddit data via [Pushshift](https://pushshift.io/).",
    sourcePath=__file__,
    datumTemplates=[subreddit_datum],
    assets=assets,
)
