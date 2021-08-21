from aorist import aorist, TextCorpusDataFromSQLite
from json import dumps

programs = {}

@aorist(
    programs,
    TextCorpusDataFromSQLite,
    entrypoint="download_text_data_from_sqlite",
    args={
        "text_attribute_name": lambda text_corpus_schema: text_corpus_schema.text_attribute_name,
        "source_tables": lambda fasttext_embedding: dumps([
            x.name() for x in fasttext_embedding.source_assets
        ]),
        "dedup_text_attribute": lambda text_corpus_schema: text_corpus_schema.should_dedup_text_attribute(),
        "db_filename": lambda fasttext_embedding: fasttext_embedding.setup.local_storage_setup.local.sq_lite_storage.location.file_name,
        "fasttext_training_data_file": lambda fasttext_embedding, context: (
            context.capture(
                "fasttext_training_data_file",
                fasttext_embedding.setup.local_storage_setup.tmp_dir + "/training_data.txt",
            ),
            context
        ),
        "tmp_dir": lambda fasttext_embedding: fasttext_embedding.setup.local_storage_setup.tmp_dir,
    },
)
def recipe(
    text_attribute_name, source_tables, fasttext_training_data_file, db_filename, tmp_dir, dedup_text_attribute,
):
    import sqlite3
    import json

    def download_text_data_from_sqlite(
        text_attribute_name, source_tables, fasttext_training_data_file, db_filename, tmp_dir, dedup_text_attribute,
    ):
     
        if not os.path.exists(tmp_dir):
            os.makedirs(tmp_dir)
        con = sqlite3.connect(db_filename)
        cursor = con.execute((chr(10) + "UNION ALL" + chr(10)).join([
            "SELECT {dedup}{text_attribute_name} FROM {source_table}".format(
                text_attribute_name=text_attribute_name,
                source_table=source_table,
                dedup="DISTINCT " if dedup_text_attribute else "",
            )
            for source_table in json.loads(source_tables)
        ]))
        with open(fasttext_training_data_file, 'w') as f: 
            for (text) in cursor:
                f.write('%s' % text)
                f.write(chr(10))
            
