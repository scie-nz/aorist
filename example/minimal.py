from scienz import subreddits
from aorist import *
from common import DEFAULT_ENDPOINTS

tmp_dir = "tmp/subreddits"
local = SQLiteStorage(
    location=SQLiteLocation(file_name="subreddits"),
    layout=StaticTabularLayout(),
)
universe = Universe(
    name="my_cluster",
    datasets=[subreddits.replicate_to_local(
        local, tmp_dir,
    )],
    endpoints=DEFAULT_ENDPOINTS,
)
print(universe.python("Replicated"))
