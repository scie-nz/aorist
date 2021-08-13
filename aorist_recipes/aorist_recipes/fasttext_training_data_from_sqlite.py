from aorist import aorist, FasttextTrainingDataFromSQLite
from json import dumps

programs = {}

@aorist(
    programs,
    FasttextTrainingDataFromSQLite,
    entrypoint="download_text_data_from_sqlite",
    args={
        "text_attribute_name": lambda fasttext_embedding_schema: fasttext_embedding_schema.text_attribute_name,
        "source_tables": lambda fasttext_embedding: dumps([
            x.name() for x in fasttext_embedding.source_assets
        ]),
        "db_filename": lambda fasttext_embedding: fasttext_embedding.setup.local_storage_setup.local.sq_lite_storage.location.file_name,
        "tmp_dir": lambda fasttext_embedding: fasttext_embedding.setup.local_storage_setup.tmp_dir
    },
)
def recipe(
    text_attribute_name, source_tables, tmp_dir, db_filename,
):
    import sqlite3
    import json

    def download_text_data_from_sqlite(text_attribute_name, source_tables, tmp_dir, db_filename):
     
        con = sqlite3.connect(db_filename)
        con.execute("\nUNION ALL\n".join([
            "SELECT {text_attribute_name} FROM {source_table}".format(
                text_attribute_name=text_attribute_name,
                source_table=source_table,
            )
            for source_table in json.loads(source_tables)
        ]))
        with open(tmp_dir + 'data.txt', 'w') as f: 
            for (text) in cursor.fetchall():
                f.write('%s\n' % text)
            
