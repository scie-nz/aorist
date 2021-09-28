from aorist import *
from aorist_recipes import programs
from scienz import (probprog, subreddit_schema)
import tempfile
from pathlib import Path
tmp = Path(tempfile.gettempdir())

local = SQLiteStorage(
    location=SQLiteLocation(file_name='subreddits.sqlite'),
    layout=TabularLayout(StaticTabularLayout()),
)
subreddits = probprog.replicate_to_local(
    Storage(local), "{}/probprog".format(tmp), Encoding(CSVEncoding())
)
universe = Universe(name="local_data", datasets=[subreddits],
                    endpoints=EndpointConfig(), compliance=None)
result = dag(universe, ["ReplicateToLocal"],
             "python", programs)

#===========================================
# Temporary fix to use tmp folder in window
result   = result.split('\n')
newlines = ['import tempfile', 'from pathlib import Path', 'tmp = Path(tempfile.gettempdir())']
result   = '\n'.join(result[0:4] + newlines + result[4:])
#===========================================

with open('generated_script.py', 'w') as f:
    f.write(result)