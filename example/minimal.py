from scienz import subreddits
from aorist import *
from common import DEFAULT_ENDPOINTS

tmp_dir = "tmp/subreddits"
local = BigQueryStorage(
    location=BigQueryLocation(),
    layout=StaticTabularLayout(),
)
universe = Universe(
    name="my_cluster",
    datasets=[subreddits],
    endpoints=DEFAULT_ENDPOINTS,
)
other_universe = Universe(
    name="my_cluster",
    datasets=[subreddits],
    endpoints=DEFAULT_ENDPOINTS,
)
print(universe.r("RDataFrame"))
