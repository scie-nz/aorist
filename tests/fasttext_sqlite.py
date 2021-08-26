from aorist import *
from aorist_recipes import programs
from scienz import (
    probprog, subreddit_schema,
    fasttext_datum, spacy_ner_datum,
    probprog,
)
    
universe = Universe(
    name="my_cluster",
    datasets=[probprog],
    endpoints=EndpointConfig(),
    compliance=None,
)
result = dag(universe, ["UploadSpacyToSQLite", "UploadFasttextToSQLite"], 
             "python", programs)
with open('generated_script_ml.py', 'w') as f:
    f.write(result)
