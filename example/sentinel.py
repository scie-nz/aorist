from aorist import (
    RowStruct,
    AlluxioLocation,
    GCSLocation,
    StaticTabularLayout,
    CSVHeader,
    GzipCompression,
    ORCEncoding,
    CSVEncoding,
    SingleFileLayout,
    RemoteStorage,
    HiveTableStorage,
    ReplicationStorageSetup,
    StaticDataTable,
    default_tabular_schema,
    DataSet,
    MinioLocation,
    attr_list,
    Universe,
)

# hacky import since submodule imports don't work well
from aorist import attributes as attr
from common import DEFAULT_ENDPOINTS

"""
Defining dataset
"""
attributes = attr_list([
    attr.KeyStringIdentifier("granule_id"),
    attr.StringIdentifier("product_id"),
    attr.StringIdentifier("datatake_identifier"),
    attr.StringIdentifier("mgrs_tile"),
    attr.POSIXTimestamp("sensing_time"),
    attr.Int64("total_size"),
    attr.Proportion("cloud_cover"),
    attr.Categorical("geometric_quality_flag"),
    attr.POSIXTimestamp("generation_time"),
    attr.FloatLatitude("north_lat", "Northern latitude of the tile's bounding box."),
    attr.FloatLatitude("south_lat", "Southern latitude of the tile's bounding box."),
    attr.FloatLatitude("west_lon", "Western longitude of the tile's bounding box."),
    attr.FloatLatitude("east_lon", "Eastern longitude of the tile's bounding box."),
    attr.URI("base_url"),
])
sentinel_granule_datum = RowStruct(
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
        header=CSVHeader(),
    ),
)
local = HiveTableStorage(
    location=MinioLocation(name="metadata"),
    layout=StaticTabularLayout(),
    encoding=ORCEncoding(),
)
sentinel_metadata_table = StaticDataTable(
    name="sentinel_metadata_table",
    schema=default_tabular_schema(sentinel_granule_datum),
    setup=ReplicationStorageSetup(
        tmp_dir="/tmp/sentinel2",
        tmp_encoding=CSVEncoding(),
        source=remote,
        targets=[local],
    ),
    tag="sentinel",
)
sentinel_dataset = DataSet(
    name="sentinel-2-dataset",
    description="Satellite data from Sentinel-2 from Google GCS",
    sourcePath=__file__,
    datumTemplates=[sentinel_granule_datum],
    assets={
        "sentinel_metadata": sentinel_metadata_table
    },
)
universe = Universe(
    name="my_cluster",
    datasets=[sentinel_dataset],
    endpoints=DEFAULT_ENDPOINTS,
)
print(universe.python("CSVIsConverted"))
