from scienz import subreddits
from aorist import *
from common import DEFAULT_ENDPOINTS

tmp_dir = "tmp/subreddits"
storage = PostgresStorage(
    location=PostgresLocation(),
    layout=StaticTabularLayout(),
)
subreddits = ['wairarapa', 'marton', 'marlborough']
assets = {x: StaticDataTable(
    name=x,
    schema=UndefinedTabularSchema(),
    setup=RemoteStorageSetup(remote=storage),
    tag=x,
) for x in subreddits}
subreddits = DataSet(
    name="subreddits",
    datumTemplates=[],
    assets=assets,
)

universe = Universe(
    name="my_cluster",
    datasets=[subreddits],
    endpoints=DEFAULT_ENDPOINTS,
)
print(universe.python("InferSchema"))
