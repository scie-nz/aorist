from aorist import aorist, UploadSpacyToSQLite

programs = {}

@aorist(
    programs,
    UploadSpacyToSQLite,
    entrypoint="upload_spacy_ner_to_sqlite",
    args={
        "db_filename": lambda named_entities: named_entities.setup.local_storage_setup.local.sq_lite_storage.location.file_name,
        "table_name": lambda named_entities: named_entities.name,
        "ner_file": lambda context: (context.get("ner_file"), context),
    },
)
def recipe(
    db_filename, table_name, ner_file,
):
    
    import sqlite3
    import json
    
    def upload_spacy_ner_to_sqlite(
        db_filename, table_name, ner_file,
    ):
        con = sqlite3.connect(db_filename)
        con.execute("DROP TABLE IF EXISTS {table_name}".format(
            table_name=table_name,
        ))
        con.execute(
        """
        CREATE TABLE {table_name}(
            line_id INTEGER,
            entity_id INTEGER,
            entity_text TEXT,
            text TEXT,
            start INTEGER,
            end INTEGER,
            label TEXT
        )
        """.format(
            table_name=table_name,
        ))

        values = []
        with open(ner_file, 'r') as f:
            for line in f.readlines():
                obj = json.loads(line)
                tpl = (
                    obj["line_id"],
                    obj["entity_id"],
                    obj["entity_text"],
                    obj["text"],
                    obj["start"],
                    obj["end"],
                    obj["label"]
                )
                values += [tpl]
                if len(values) % 100 == 0:
                    con.executemany(
                        """
                        INSERT INTO {table_name}(
                            line_id, entity_id, entity_text,
                            text, start, end, label
                        ) VALUES (?, ?, ?, ?, ?, ?, ?)
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
                INSERT INTO {table_name}(
                    line_id, entity_id, entity_text,
                    text, start, end, label
                ) VALUES (?, ?, ?, ?, ?, ?, ?)
                """.format(
                    table_name=table_name,
                ),
                values
            )
            con.commit()
        count = list(con.execute("SELECT COUNT(*) FROM " + table_name))[0][0]
        print("Inserted %d records into %s" % (count, table_name))
        row = list(con.execute("SELECT * FROM " + table_name + " ORDER BY RANDOM() LIMIT 1"))[0]
        print("Example record:")
        for x in zip(["line_id", "entity_id", "entity_text", "text", "start", "end", "label"], row):
            print("%s: %s" % x)
        con.close()
