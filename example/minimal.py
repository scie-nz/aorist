from scienz import covid
from aorist import *
from common import DEFAULT_ENDPOINTS

tmp_dir = "tmp/covid"
local = BigQueryStorage(
    location=BigQueryLocation(),
    layout=StaticTabularLayout(),
)
universe = Universe(
    name="my_cluster",
    datasets=[covid.replicate_to_local(
        local, tmp_dir, CSVEncoding(),
    )],
    endpoints=DEFAULT_ENDPOINTS,
)
other_universe = Universe(
    name="my_cluster",
    datasets=[covid],
    endpoints=DEFAULT_ENDPOINTS,
)
#print(universe.python("RemoveFileHeader"))
print(universe.python("NumpyFromCSVData"))
