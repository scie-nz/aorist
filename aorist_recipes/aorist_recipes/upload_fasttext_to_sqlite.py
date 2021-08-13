from aorist import aorist, UploadFasttextToSQLite

programs = {}

@aorist(
    programs,
    UploadFasttextToSQLite,
    entrypoint="upload_fasttext_to_sqlite",
    args={
        "db_filename": lambda fasttext_embedding: fasttext_embedding.setup.local_storage_setup.local.sq_lite_storage.location.file_name,
        "tmp_dir": lambda fasttext_embedding: fasttext_embedding.setup.local_storage_setup.tmp_dir,
        "table_name": lambda fasttext_embedding: fasttext_embedding.name,
    },
)
def recipe(
    db_filename, table_name, tmp_dir, 
):
    
    import sqlite3
    import json
    
    def upload_fasttext_to_sqlite(
        db_filename, table_name, tmp_dir,
    ):
        con = sqlite3.connect(db_filename)
        con.execute("""
        DROP TABLE IF EXISTS {table_name}
        """.format(
            table_name=table_name,
        ))
        con.execute("""
        CREATE TABLE {table_name}(word TEXT, embedding TEXT)
        """.format(
            table_name=table_name,
        ))

        values = []
        with open(tmp_dir + '/words.txt', 'r') as f:
            for line in f.readlines():
                splits = line.strip().split("\t")
                assert len(splits) == 2
                tpl = tuple(splits)
                values += [tpl]
                if len(values) % 100 == 0:
                    con.executemany(
                        """
                        INSERT INTO {table_name}(word, embedding) VALUES ("?", "?")
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
                INSERT INTO {table_name}(word, embedding) VALUES ("?", "?")
                """.format(
                    table_name=table_name,
                ),
                values
            )
            con.commit()
        count = list(con.execute("SELECT COUNT(*) FROM " + table_name))[0][0]
        print("Inserted %d records into %s" % (count, table_name))
        con.close()
