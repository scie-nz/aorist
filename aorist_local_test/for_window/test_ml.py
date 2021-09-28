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

# Temporary fix to use tmp folder in window
result   = result.split('\n')
newlines = ['import tempfile', 'from pathlib import Path', 'tmp = Path(tempfile.gettempdir())']
result   = '\n'.join(result[0:5] + newlines + result[5:])

with open('generated_script_ml.py', 'w') as f:
    f.write(result)


# result = dag(universe, ["UploadFasttextToSQLite"], 
#              "jupyter", programs)
# with open('generated_notebook.ipynb', 'w') as f:
#     f.write(result)


# result = dag(universe, ["UploadFasttextToSQLite"], 
#              "airflow", programs)
# with open('generated_script_airflow.py', 'w') as f:
#     f.write(result)