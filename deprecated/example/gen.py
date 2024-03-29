import sys
from aorist import (
    dag,
    Universe,
    ComplianceConfig,
    HiveTableStorage,
    MinioLocation,
    StaticTabularLayout,
    ORCEncoding,
)
from common import DEFAULT_USERS, DEFAULT_GROUPS, DEFAULT_ENDPOINTS

# from sentinel import sentinel_dataset
from scienz import covid

universe = Universe(
    name="my_cluster",
    users=DEFAULT_USERS,
    groups=DEFAULT_GROUPS,
    datasets=[
        covid,
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
out = dag(universe, [
    "AllAssetsComputed",
], sys.argv[1])
print(out.replace("\\\\", "\\"))

# universe.derive_asset(
#     """
#     SELECT *
#     FROM wine.wine_table
#     WHERE wine.wine_table.alcohol > 14.0
#     """,
#     name="high_abv_wines",
#     storage=HiveTableStorage(
#         location=MinioLocation(name="high_abv_wines"),
#         layout=StaticTabularLayout(),
#         encoding=ORCEncoding(),
#     ),
#     tmp_dir="/tmp/high_abv_wines",
# )
# universe.derive_asset(
#     """
#     SELECT *
#     FROM wine.wine_table
#     WHERE wine.wine_table.alcohol <= 14.0
#     """,
#     name="lower_abv_wines",
#     storage=HiveTableStorage(
#         location=MinioLocation(name="lower_abv_wines"),
#         layout=StaticTabularLayout(),
#         encoding=ORCEncoding(),
#     ),
#     tmp_dir="/tmp/high_abv_wines",
# )
