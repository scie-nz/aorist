from aorist import (
    dag,
    Universe,
)
from common import DEFAULT_USERS, DEFAULT_GROUPS, DEFAULT_ENDPOINTS
from sentinel import sentinel_dataset
from snap import snap_dataset
from geonames import geonames_dataset

universe = Universe(
    name="my_cluster",
    users=DEFAULT_USERS,
    groups=DEFAULT_GROUPS,
    datasets=[
        snap_dataset,
    ],
    endpoints=DEFAULT_ENDPOINTS,
)
out = dag(universe, ["Test"], "python")
print(out.replace("\\\\","\\"))
