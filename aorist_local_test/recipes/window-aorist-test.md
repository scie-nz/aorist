# Conda-test for Aorist on Window 10 (Local)

## Testing preparation
### 1. Conda environment preparation
```python
# Create new conda environemtn with python 3.7
conda create -n aorist-test -c scienz -c conda-forge aorist
conda activate aorist-test
```

### 2. Setting up local `scienz` and `aorist_recipes`
Since up-to-date version of `scienz` and `aorist_recipes` libraries have not be published, testing uses local  `scienz` and `aorist_recipes` inside `aorist` cloned from Github.   

```python
git clone git@github.com:scie-nz/aorist.git
```
If you are not familiar with Github on Linux, please compress your `aorist` library and copy it to your window system. Assuming that `aorist` is cloned at C (Local Disk) `C:/aorist`. 
- Create a new folder `C:/aorist_local_test`
- Copy `aorist_recipes` from C:/aorist/aorist_recipes/`aorist_recipes` (cloned folder) and paste to the folder C:/aorist_local_test/`aorist_recipes` (new created folder) 
- Copy `scienz` from C:/aorist/scienz/`scienz` (cloned folder) and paste to folder C:/aorist_local_test/`scienz` (new created folder) 


Your `aorist_local_test` look like:
```python
C:/aorist_local_test/
                    aorist_recipes
                    scienz
```
### 3. Install a few required pip packages for the demo
```python
pip install praw pmaw fasttext
```
## Minimal Example
### 1. Try it on a test script called test.py:
Create `test.py` inside `aorist_local_test`
```python
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
```
### 2. Run the script with
```python
cd C:/aorist_local_test
python test.py
```
### 3. A new script `generated_script.py` will be generated
The `generated_script.py` needs to be modified at path with `tmp`:

FROM: 
- `tmp_dir="/tmp/probprog",`
- `output_file="/tmp/probprog/probprog.json",`
- `source_file="/tmp/probprog/probprog.json",`

TO:
- `tmp_dir="{}/probprog".format(tmp),`
- `output_file="{}/probprog/probprog.json".format(tmp),`
- `source_file="{}/probprog/probprog.json".format(tmp),`

to change the path of temporary folder in window.

Run `generated_script.py`.
```python
python generated_script.py
```

### 4. Running the generated script should result in something like:
```python
Inserted 292 records into probprog
Example record
id: 3cwwj2
author: pfumbi
subreddit: probprog
created_utc: 1436624012
title: Sigma is a probabilistic programming environment implemented in Julia
selftext:
```

## Machine Learning Example
### 1. Write the following script (called `test_ml.py`) -- this script will generate our code:
The line `tmp = Path(tempfile.gettempdir())` works in test.py file but sometimes crashes in generating `generated_script_ml.py` file, especially window users who have their username starting at "n" like me because it create the new path with `"\"`, not `"/"`. The line `"tmp = '/'.join(tempfile.gettempdir().split('\\'))"` can temporarily fix it. 
```python
from aorist import *
from aorist_recipes import programs
from scienz import (
    probprog, subreddit_schema
)
import tempfile
from pathlib import Path
# tmp = Path(tempfile.gettempdir()) 
tmp = '/'.join(tempfile.gettempdir().split('\\'))

fasttext_attributes = [
    Attribute(KeyStringIdentifier("word_id")),
    Attribute(FreeText("word")),
    Attribute(FreeText("embedding")),
]
fasttext_datum = RowStruct(
    name="fasttext",
    attributes=fasttext_attributes,
)

local = SQLiteStorage(
    location=SQLiteLocation(file_name='probprog.sqlite'),
    layout=TabularLayout(StaticTabularLayout()),
)
subreddits = probprog.replicate_to_local(
    Storage(local), "{}/probprog".format(tmp), Encoding(CSVEncoding())
)
source_assets = list(subreddits.assets.values())
text_template = DatumTemplate(Text(name="corpus"))
text_corpus_schema = TextCorpusSchema(
    sources=[x.static_data_table for x in source_assets if x.static_data_table is not None],
    datum_template=text_template,
    text_attribute_name="title",
)
text_corpus = TextCorpus(
    name="text_corpus",
    comment="Subreddit text corpus",
    schema=DataSchema(LanguageAssetSchema(text_corpus_schema)),
    setup=StorageSetup(LocalStorageSetup(
        Storage(local),
        '{}/probprog'.format(tmp),
    )),
)
subreddits.add_asset(Asset(LanguageAsset(text_corpus)))

embedding = FasttextEmbedding(
    name="embedding",
    comment="Fasttext embedding of size 128",
    schema=DataSchema(LanguageAssetSchema(FasttextEmbeddingSchema(
        dim=16,
        source=text_corpus,
        datum_template=DatumTemplate(fasttext_datum)
    ))),
    setup=StorageSetup(LocalStorageSetup(
        Storage(local),
        '{}/probprog'.format(tmp),
    )),
)
subreddits.add_asset(Asset(LanguageAsset(embedding)))

#subreddits.add_asset('embedding', Asset(embedding))
universe = Universe(
    name="my_cluster",
    datasets=[subreddits],
    endpoints=EndpointConfig(),
    compliance=None,
)
result = dag(universe, ["UploadFasttextToSQLite"], 
             "python", programs)

#===========================================
# Temporary fix to use tmp folder in window (Hoang Added)
result   = result.split('\n')
newlines = ['import tempfile', 'from pathlib import Path', 'tmp = Path(tempfile.gettempdir())']
result   = '\n'.join(result[0:5] + newlines + result[5:])
#===========================================

with open('generated_script_ml.py', 'w') as f:
    f.write(result)
```
### 2. Run the codegen script:
```python
python test_ml.py
```
### 3. Then run the generated code:
The `generated_script_ml.py` needs to be modified at path with `tmp`:

FROM:
- `tmp_dir="/tmp/probprog",`
- `output_file="/tmp/probprog/probprog.json",`
- `source_file="/tmp/probprog/probprog.json",`

TO:
- `tmp_dir="{}/probprog".format(tmp),`
- `output_file="{}/probprog/probprog.json".format(tmp),`
- `source_file="{}/probprog/probprog.json".format(tmp),`
to change the path of temporary folder in window.

The line 24 of the function `download_text_data_from_sqlite` in the script requires minor modification by adding `encoding="utf-8"` inside `open(...)`.
- Originally generated script (Line 24 of the function): 
```python 
...
with open(text_data_file, "w") as f:
    ...
```
- Modified script (Line 24): 
```python 
...
with open(text_data_file, "w",  encoding="utf-8") as f:
    ...
```
After modifying the `generated_script_ml.py` file, run the command below

```python
python generated_script_ml.py
```
The file `word_embeddings.txt` wil be created in `C:/Users/username/AppData/Local/Temp/probprog`. 

The output file looks like this
```python
{"id": 0, "word": "</s>", "embedding": [-0.05646619200706482, 0.05358843132853508, -0.039630308747291565, -0.012063123285770416, 0.030040951445698738, 0.006308949086815119, 0.04041805863380432, 3.3989534131251276e-05, 0.06578253954648972, 0.062115628272295, -0.010872391983866692, 0.017077114433050156, 0.059902530163526535, 0.049544431269168854, -0.02923724427819252, -0.014039932750165462]}
{"id": 1, "word": "Probabilistic", "embedding": [-0.010567756369709969, 0.004026586189866066, 0.0025988752022385597, -0.003570996690541506, 0.0034755226224660873, -0.01178531814366579, 0.004103485494852066, 0.0053864093497395515, 0.009040310978889465, 0.0015766965225338936, -0.0007015346782281995, -0.002070460468530655, -0.005370985250920057, 0.00937122106552124, -0.004693705588579178, -0.0067557692527771]}

...

{"id": 52, "word": "Programming:", "embedding": [-0.003214445896446705, 0.006667036097496748, -0.0011871808674186468, -0.0004227895988151431, -0.002411183202639222, -0.00915229320526123, 2.8761493013007566e-05, 0.001706317882053554, 0.010234243236482143, 0.006583030801266432, 0.0013282165164127946, -0.004983318038284779, 0.008693241514265537, 0.002687988104298711, -0.01589180715382099, -0.004517096094787121]}
{"id": 53, "word": "Probability", "embedding": [-0.010916689410805702, 0.003262341022491455, -0.0009410164784640074, -0.004229385871440172, 0.007250891998410225, -0.007714967243373394, 0.003956979140639305, 0.004869392141699791, 0.012761253863573074, -0.0031990937422960997, 0.005877084564417601, -0.0007109571597538888, -0.010415489785373211, 0.009788807481527328, -0.006410091649740934, -0.00770526984706521]}

```