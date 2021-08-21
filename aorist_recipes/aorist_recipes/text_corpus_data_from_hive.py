from aorist import aorist, TextCorpusDataFromHive
from json import dumps

programs = {}

@aorist(
    programs,
    TextCorpusDataFromHive,
    entrypoint="download_text_data_from_trino",
    args={
        "host": lambda universe: universe.endpoints.presto.server, 
        "user": lambda universe: universe.endpoints.presto.user, 
        "port": lambda universe: str(universe.endpoints.presto.http_port), 
        "text_attribute_name": lambda fasttext_embedding_schema: fasttext_embedding_schema.text_attribute_name,
        "source_tables": lambda fasttext_embedding: dumps([
            x.name() for x in fasttext_embedding.source_assets
        ]),
        "tmp_dir": lambda fasttext_embedding: fasttext_embedding.setup.local_storage_setup.tmp_dir
    },
)
def recipe(
    host, user, port, text_attribute_name, source_tables, tmp_dir,
):
    import os
    import urllib.request
    import json

    def download_text_data_from_trino(host, user, port, text_attribute_name, source_tables, tmp_dir):
     
        connection = trino.dbapi.connect(
            host=host,
            user=user,
            port=int(port),
            catalog='hive',
        )
        cursor = connection.cursor()
        cursor.execute("\nUNION ALL\n".join([
            "SELECT {text_attribute_name} FROM {source_table}".format(
                text_attribute_name=text_attribute_name,
                source_table=source_table,
            )
            for source_table in json.loads(source_tables)
        ]))
        with open(tmp_dir + 'data.txt', 'w') as f: 
            for (text) in cursor.fetchall():
                f.write('%s\n' % text)
            
