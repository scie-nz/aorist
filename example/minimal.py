from scienz import snap
from aorist import *
from common import DEFAULT_ENDPOINTS

tmp_dir = "tmp/snap"
local = HiveTableStorage(
    location=MinioLocation(name="wine"),
    layout=StaticHiveTableLayout(),
    encoding=ORCEncoding(),
)
universe = Universe(
    name="my_cluster",
    datasets=[snap.replicate_to_local(
        local, tmp_dir,
    )],
    endpoints=DEFAULT_ENDPOINTS,
)
print(universe.jupyter("Replicated"))
