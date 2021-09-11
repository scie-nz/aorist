from aorist import *
from .subreddits import (
    place_based_subreddits, us_subreddits, nz_subreddits, datascience, datamining, probprog,
    subreddit_schema, subreddit_datum, 
)
from .subreddits import build_assets as build_subreddit_assets
from .sports import *

fasttext_attributes = [
    Attribute(KeyStringIdentifier("word_id")),
    Attribute(FreeText("word")),
    Attribute(FreeText("embedding")),
]
fasttext_datum = RowStruct(
    name="fasttext",
    attributes=fasttext_attributes,
)

spacy_ner_attributes = [
    Attribute(Int64Identifier("line_id")),
    Attribute(KeyStringIdentifier("entity_id")),
    Attribute(FreeText("entity_text")),
    Attribute(FreeText("text")),
    Attribute(Count("start")),
    Attribute(Count("end")),
    Attribute(Factor("label")),
]
spacy_ner_datum = RowStruct(
    name="spacy_ner",
    attributes=spacy_ner_attributes,
)
