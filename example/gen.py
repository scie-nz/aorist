from aorist import (
    dag,
    Universe,
    ComplianceConfig,
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
        snap_dataset
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
    "DataDownloadedAndConverted", "IsAudited",
], "airflow")
print(out.replace("\\\\", "\\"))
