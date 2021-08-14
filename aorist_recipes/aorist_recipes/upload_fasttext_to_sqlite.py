from aorist import aorist, UploadFasttextToSQLite

programs = {}

@aorist(
    programs,
    UploadFasttextToSQLite,
    entrypoint="upload_fasttext_to_sqlite",
    args={
        "db_filename": lambda fasttext_embedding: fasttext_embedding.setup.local_storage_setup.local.sq_lite_storage.location.file_name,
        "table_name": lambda fasttext_embedding: fasttext_embedding.name,
        "fasttext_word_embeddings_file": lambda context: (context.get("fasttext_word_embeddings_file"), context),
    },
)
def recipe(
    db_filename, table_name, fasttext_word_embeddings_file, 
):
    
    import sqlite3
    import json
    
    def upload_fasttext_to_sqlite(
        db_filename, table_name, fasttext_word_embeddings_file,
    ):
        con = sqlite3.connect(db_filename)
        con.execute("""
        DROP TABLE IF EXISTS {table_name}
        """.format(
            table_name=table_name,
        ))
        con.execute("""
        CREATE TABLE {table_name}(word_id INTEGER, word TEXT, embedding TEXT)
        """.format(
            table_name=table_name,
        ))

        values = []
        with open(fasttext_word_embeddings_file, 'r') as f:
            for line in f.readlines():
                obj = json.loads(line)
                tpl = (obj["id"], obj["word"], json.dumps(obj["embedding"]))
                values += [tpl]
                if len(values) % 100 == 0:
                    con.executemany(
                        """
                        INSERT INTO {table_name}(word_id, word, embedding) VALUES (?, ?, ?)
                        """.format(
                            table_name=table_name,
                        ),
                        values
                    )
                    con.commit()
                    values = []
        
        if len(values) > 0:
            con.executemany(
                """
                INSERT INTO {table_name}(word_id, word, embedding) VALUES (?, ?, ?)
                """.format(
                    table_name=table_name,
                ),
                values
            )
            con.commit()
        count = list(con.execute("SELECT COUNT(*) FROM " + table_name))[0][0]
        print("Inserted %d records into %s" % (count, table_name))
        con.close()
