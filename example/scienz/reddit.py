
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
    JSONEncoding,
)
from aorist import attributes as attr

attributes = attr_list([
    attr.KeyStringIdentifier("post_id"),
    attr.StringIdentifier("subreddit"),
    attr.POSIXTimestamp("created_utc"),
    attr.FreeText("text"),
])
subreddit_datum = RowStruct(
    name="subreddit",
    attributes=attributes,
)
wairarapa = StaticDataTable(
    name='wairarapa',
    schema=default_tabular_schema(subreddit_datum),
    setup=RemoteStorageSetup(
        remote=RemoteStorage(
            location=PushshiftAPILocation(subreddit='wairarapa'),
            layout=PushshiftSubredditPostsAPILayout(),
            encoding=JSONEncoding(),
        ),
    ),
    tag='r_wairarapa',
)

subreddits = DataSet(
    name="subreddits",
    datumTemplates=[subreddit_datum],
    assets={"wairarapa": wairarapa},
)
