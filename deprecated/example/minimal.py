from scienz import subreddits
from aorist import *
from common import DEFAULT_ENDPOINTS

tmp_dir = "tmp/subreddits"
local = BigQueryStorage(
    location=BigQueryLocation(),
    layout=StaticTabularLayout(),
)
subreddits = subreddits.replicate_to_local(local, tmp_dir, CSVEncoding())
universe = Universe(
    name="my_cluster",
    datasets=[subreddits],
    endpoints=DEFAULT_ENDPOINTS,
)
print(universe.python("PandasData"))

