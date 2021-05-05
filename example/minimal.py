from scienz import snap
from aorist import *
from common import DEFAULT_ENDPOINTS

tmp_dir = "tmp/snap"
local = BigQueryStorage(
    location=BigQueryLocation(),
    layout=StaticTabularLayout(),
)
universe = Universe(
    name="my_cluster",
    datasets=[snap.replicate_to_local(
        local, tmp_dir, CSVEncoding(),
    )],
    endpoints=DEFAULT_ENDPOINTS,
)
other_universe = Universe(
    name="my_cluster",
    datasets=[snap],
    endpoints=DEFAULT_ENDPOINTS,
)
print(universe.r("RDataFrameFromCSVData"))
