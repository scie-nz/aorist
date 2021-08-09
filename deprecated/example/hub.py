from scienz import covid
from aorist import *
from common import DEFAULT_ENDPOINTS

universe = Universe(
    name="scienz",
    datasets=[covid],
    endpoints=DEFAULT_ENDPOINTS,
)
print(universe.python("GenerateHub"))
