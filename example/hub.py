from scienz import subreddits
from aorist import *
from common import DEFAULT_ENDPOINTS

universe = Universe(
    name="scienz",
    datasets=[subreddits],
    endpoints=DEFAULT_ENDPOINTS,
)
print(universe.python("GenerateMarkdownForDatasets"))
