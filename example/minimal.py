from scienz import wine
from aorist import *
from common import DEFAULT_ENDPOINTS

tmp_dir = "tmp/wine"
local = BigQueryStorage(
    location=BigQueryLocation(),
    layout=StaticTabularLayout(),
)
universe = Universe(
    name="my_cluster",
    datasets=[wine.replicate_to_local(
        local, tmp_dir, CSVEncoding(),
    )],
    endpoints=DEFAULT_ENDPOINTS,
)
other_universe = Universe(
    name="my_cluster",
    datasets=[wine],
    endpoints=DEFAULT_ENDPOINTS,
)
print(universe.python("PandasFromCSVData"))
print(universe.python("NumpyFromCSVData"))
