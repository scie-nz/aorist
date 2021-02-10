from aorist import (
    KeyedStruct,
    AlluxioLocation,
    GCSLocation,
    StaticHiveTableLayout,
    UpperSnakeCaseCSVHeader,
    GzipCompression,
    ORCEncoding,
    CSVEncoding,
    SingleFileLayout,
    RemoteStorage,
    HiveTableStorage,
    RemoteImportStorageSetup,
    StaticDataTable,
    default_tabular_schema,
    DataSet,
)

# hacky import since submodule imports don't work well
from aorist import attributes as attr

"""
Defining dataset
"""
attributes = [
    attr.KeyStringIdentifier("granule_id"),
    attr.NullableStringIdentifier("product_id"),
    attr.NullableStringIdentifier("datatake_identifier"),
    attr.NullableStringIdentifier("mgrs_tile"),
    attr.NullablePOSIXTimestamp("sensing_time"),
    attr.NullableInt64("total_size"),
    attr.NullableString("cloud_cover"),
    attr.NullableString("geometric_quality_flag"),
    attr.NullablePOSIXTimestamp("generation_time"),
    attr.FloatLatitude("north_lat", "Northern latitude of the tile's bounding box."),
    attr.FloatLatitude("south_lat", "Southern latitude of the tile's bounding box."),
    attr.FloatLatitude("west_lon", "Western longitude of the tile's bounding box."),
    attr.FloatLatitude("east_lon", "Eastern longitude of the tile's bounding box."),
    attr.URI("base_url"),
]
sentinel_granule_datum = KeyedStruct(
    name="sentinel_granule_datum",
    attributes=attributes,
)
remote = RemoteStorage(
    location=GCSLocation(
        bucket="gcp-public-data-sentinel-2",
        blob="index.csv.gz",
    ),
    layout=SingleFileLayout(),
    encoding=CSVEncoding(
        compression=GzipCompression(),
        header=UpperSnakeCaseCSVHeader(),
    ),
)
local = HiveTableStorage(
    location=AlluxioLocation("sentinel2", "metadata"),
    layout=StaticHiveTableLayout(),
    encoding=ORCEncoding(),
)
sentinel_metadata_table = StaticDataTable(
    name="sentinel_metadata_table",
    schema=default_tabular_schema(sentinel_granule_datum),
    setup=RemoteImportStorageSetup(
        tmp_dir="/tmp/sentinel2",
        remote=remote,
        local=[local],
    ),
    tag="sentinel",
)
sentinel_dataset = DataSet(
    name="sentinel-2-dataset",
    datumTemplates=[sentinel_granule_datum],
    assets=[sentinel_metadata_table],
)
