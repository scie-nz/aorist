from aorist import *
from aorist_recipes import programs
from scienz import (probprog, subreddit_schema)

local = SQLiteStorage(
    location=SQLiteLocation(file_name='subreddits.sqlite'),
    layout=TabularLayout(StaticTabularLayout()),
)
subreddits = probprog.replicate_to_local(
    Storage(local), "/tmp/probprog", Encoding(CSVEncoding())
)
universe = Universe(name="local_data", datasets=[subreddits],
                    endpoints=EndpointConfig(), compliance=None)
result = dag(universe, ["ReplicateToLocal"],
             "python", programs)
with open('generated_script.py', 'w') as f:
    f.write(result)
