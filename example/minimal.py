from scienz import subreddits
from aorist import *
from common import DEFAULT_ENDPOINTS

tmp_dir = "tmp/subreddits"
local = HiveTableStorage(
    location=MinioLocation(name="wine"),
    layout=StaticHiveTableLayout(),
    encoding=ORCEncoding(),
)
universe = Universe(
    name="my_cluster",
    datasets=[subreddits.replicate_to_local(
        local, tmp_dir,
    )],
    endpoints=DEFAULT_ENDPOINTS,
)
print(universe.python("Replicated"))
