from aorist import (
    dag,
    Universe,
    ComplianceConfig,
    HiveTableStorage,
    MinioLocation,
    StaticHiveTableLayout,
    ORCEncoding,
)
from common import DEFAULT_USERS, DEFAULT_GROUPS, DEFAULT_ENDPOINTS

# from sentinel import sentinel_dataset
from snap import snap_dataset
from geonames import geonames_dataset, geonames_table
from wine import wine_dataset

universe = Universe(
    name="my_cluster",
    users=DEFAULT_USERS,
    groups=DEFAULT_GROUPS,
    datasets=[
        wine_dataset,
    ],
    endpoints=DEFAULT_ENDPOINTS,
    compliance=ComplianceConfig(
        description="""
        Testing workflow for data replication of SNAP data to
        local cluster. The SNAP dataset collection is provided
        as open data by Stanford University. The collection contains
        various social and technological network graphs, with
        reasonable and systematic efforts having been made to ensure
        the removal of all Personally Identifiable Information.
        """,
        data_about_human_subjects=True,
        contains_personally_identifiable_information=False,
    ),
)
universe.derive_asset(
    """
    SELECT *
    FROM wine.wine_table
    WHERE wine.wine_table.alcohol > 14.0
    """,
    name="high_abv_wines",
    storage=HiveTableStorage(
        location=MinioLocation(name="high_abv_wines"),
        layout=StaticHiveTableLayout(),
        encoding=ORCEncoding(),
    ),
    tmp_dir="/tmp/high_abv_wines",
)
universe.derive_asset(
    """
    SELECT *
    FROM wine.wine_table
    WHERE wine.wine_table.alcohol <= 14.0
    """,
    name="lower_abv_wines",
    storage=HiveTableStorage(
        location=MinioLocation(name="lower_abv_wines"),
        layout=StaticHiveTableLayout(),
        encoding=ORCEncoding(),
    ),
    tmp_dir="/tmp/high_abv_wines",
)
out = dag(universe, [
    "DataDownloadedAndConverted",
], "python")
print(out.replace("\\\\", "\\"))
