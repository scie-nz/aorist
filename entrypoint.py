from aorist import (
    User,
    EndpointConfig,
    AlluxioConfig,
    PrestoConfig,
    RangerConfig,
    GiteaConfig,
    UserGroup,
    GlobalPermissionsAdmin,
    KeyedStruct,
    TabularSchema,
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
    Universe,
)

# hacky import since submodule imports don't work well
from aorist import attributes as attr

"""
Defining endpoints.
"""
alluxio_config = AlluxioConfig(server="alluxio")
ranger_config = RangerConfig(
    server="localhost", user="admin", password="G0powerRangers"
)
presto_config = PrestoConfig(server="presto-coordinator-0")
gitea_config = GiteaConfig(token="2b44b07e042ee9fe374e3eeebd2c9098468b5774")
endpoint_config = EndpointConfig(
    alluxio=alluxio_config,
    ranger=ranger_config,
    presto=presto_config,
    gitea=gitea_config,
)

"""
Defining roles
"""
global_permissions_admin = GlobalPermissionsAdmin()

"""
Defining users.
"""
bogdan = User(
    firstName="Bogdan",
    lastName="State",
    email="bogdan@scie.nz",
    unixname="bogdan",
    roles=[global_permissions_admin],
)
nick = User(
    firstName="Nick", lastName="Parker", email="nick@scie.nz", unixname="nick"
)
cip = User(
    firstName="Ciprian", lastName="Gerea", email="cip@scie.nz", unixname="cip"
)

"""
Defining user groups
"""

finance = UserGroup(
    name="finance-users", users=[bogdan], labels={"department": "finance"}
)
datascience = UserGroup(
    name="finance-users",
    users=[bogdan, nick, cip],
    labels={"department": "datascience"},
)
crowding = UserGroup(
    name="project-crowding-detection",
    users=[bogdan],
    labels={"project": "crowding_detection"},
)

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
    attr.FloatLatitude(
        "north_lat", "Northern latitude of the tile's bounding box."
    ),
    attr.FloatLatitude(
        "south_lat", "Southern latitude of the tile's bounding box."
    ),
    attr.FloatLatitude(
        "west_lon", "Western longitude of the tile's bounding box."
    ),
    attr.FloatLatitude(
        "east_lon", "Eastern longitude of the tile's bounding box."
    ),
    attr.URI("base_url"),
]
sentinel_granule_datum = KeyedStruct(
    name="sentinel_granule_datum",
    attributes=attributes,
)
remote = RemoteStorage(
    location=GCSLocation(
        bucket="gcp-public-data-sentinel2",
        blob="index.csv.gz-backup",
    ),
    layout=SingleFileLayout(),
    encoding=CSVEncoding(
        compression=GzipCompression(),
        header=UpperSnakeCaseCSVHeader(),
    ),
)
local = HiveTableStorage(
    location=AlluxioLocation("sentinel2/metadata"),
    layout=StaticHiveTableLayout(),
    encoding=ORCEncoding(),
)
sentinel = StaticDataTable(
    name='sentinel',
    schema=TabularSchema(sentinel_granule_datum),
    setup=RemoteImportStorageSetup(
        tmp_dir="/tmp/sentinel2",
        remote=remote,
        local=[local],
    )
)
