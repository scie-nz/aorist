from scienz import wine
from aorist import (
    dag, Universe, HiveTableStorage,
    MinioLocation, StaticHiveTableLayout, ORCEncoding,
)
from common import DEFAULT_ENDPOINTS

local = HiveTableStorage(
    location=MinioLocation(name="wine"),
    layout=StaticHiveTableLayout(),
    encoding=ORCEncoding(),
)
wine_dataset = wine.replicate_to_local(
    tmp_dir="/tmp/wine",
    storage=local,
)
universe = Universe(
    name="my_cluster",
    datasets=[wine_dataset],
    endpoints=DEFAULT_ENDPOINTS,
)

out = dag(universe, ["Replicated"], "jupyter")
print(out)
