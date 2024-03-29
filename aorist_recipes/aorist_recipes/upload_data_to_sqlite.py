from aorist import aorist, UploadDataToSQLite, UploadSpaCyToSQLite, UploadFasttextToSQLite
from json import dumps

programs = {}

@aorist(
    programs,
    [UploadDataToSQLite, UploadSpaCyToSQLite, UploadFasttextToSQLite],
    entrypoint="upload_to_sqlite",
    args={
        "db_filename": lambda asset: asset.storage_setup.local[0].sq_lite_storage.location.file_name,
        "table_name": lambda asset: asset.name,
        "header_num_lines": lambda context: (context.get_int("header_num_lines"), context),
        "is_json": lambda context: (context.get_bool("is_json"), context),
        "delimiter": lambda context: (context.get("delimiter"), context),
        "source_file": lambda context: (context.get("file_to_replicate"), context),
        "columns": lambda asset: [
            (x.name, x.sqlite_type, x.is_nullable)
            for x in asset.schema.datum_template.attributes()
        ],
    }
)
def recipe(
    db_filename, table_name, source_file, columns,
    is_json, delimiter, header_num_lines,
):
    
    import sqlite3
    import json
    
    def upload_to_sqlite(
        db_filename, table_name, source_file, columns,
        is_json, delimiter, header_num_lines,
    ):
        con = sqlite3.connect(db_filename)
        con.execute("DROP TABLE IF EXISTS {table_name}".format(
            table_name=table_name,
        ))
        con.execute(
            "CREATE TABLE {table_name}({schema})".format(
                table_name=table_name,
                schema=",\\n".join(["%s %s" % (k, v) for k, v, _ in columns]),
            )
        )

        values = []
        type_fn = []
        for _, v, _ in columns:
            if v == 'TEXT':
                type_fn += [lambda x: str(x)]
            elif v == 'INTEGER':
                type_fn += [lambda x: int(x) if x is not None else None]
            elif v == 'REAL':
                type_fn += [float]
            elif v == 'BLOB':
                type_fn += [bytes]
            else:
                type_fn += [lambda _: None]

        attr_names = [x[0] for x in columns]
        with open(source_file, 'r') as f:
            for line in f.readlines()[header_num_lines:]:
                if is_json:
                    x = json.loads(line)
                    obj = [x[name] if name in x else None for name in attr_names]
                else:
                    obj = line.split(delimiter)
                    assert len(obj) == len(type_fn), (obj, type_fn)
                tpl = tuple(fn(arg) for fn, arg in zip(type_fn, obj))
                values += [tpl]

        con.executemany(
            "INSERT INTO {table_name}({columns}) VALUES ({vals})".format(
                table_name=table_name,
                columns=", ".join([k for k, _, _ in columns]),
                vals=", ".join(["?"] * len(columns))
            ),
            values
        )
        con.commit()
        count = list(con.execute("SELECT COUNT(*) FROM " + table_name))[0][0]
        print("Inserted %d records into %s" % (count, table_name))
        row = list(con.execute("SELECT * FROM " + table_name + " ORDER BY RANDOM() LIMIT 1"))[0]
        print("Example record")
        for x in zip(attr_names, row):
            print("%s: %s" % x)
        con.close()
