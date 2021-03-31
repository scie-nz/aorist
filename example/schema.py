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
    description="Example schema storage",
    datumTemplates=[],
    assets={},
)
universe = Universe(
    name="my_cluster",
    datasets=[subreddits],
    endpoints=DEFAULT_ENDPOINTS,
)
datum_templates = {}
print(universe.jupyter("InferSchema"))
