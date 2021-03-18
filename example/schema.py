from scienz import subreddits
from aorist import *
from common import DEFAULT_ENDPOINTS

tmp_dir = "tmp/subreddits"
storage = PostgresStorage(
    location=PostgresLocation(),
    layout=StaticTabularLayout(),
)
subreddits = DataSet(
    name="subreddits",
    datumTemplates=[],
    assets={},
)
universe = Universe(
    name="my_cluster",
    datasets=[subreddits],
    endpoints=DEFAULT_ENDPOINTS,
)
datum_templates = {}
print(universe.python("InferSchema"))
