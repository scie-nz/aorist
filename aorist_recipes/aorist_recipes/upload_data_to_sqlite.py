from aorist import aorist, UploadDataToSQLite
from json import dumps

programs = {}

@aorist(
    programs,
    UploadDataToSQLite,
    entrypoint="upload_to_sqlite",
    args={
        "db_filename": lambda sq_lite_location: sq_lite_location.file_name,
        "table_name": lambda static_data_table: static_data_table.name,
        "source_file": lambda context: (context.get("json_file"), context),
        "columns": lambda data_set, asset: dumps([
            (x.name, x.sqlite_type, x.is_nullable)
            for x in data_set.get_template(asset).attributes()
        ]),
    }
)
def recipe(
    db_filename, table_name, source_file, columns,
):
    
    import sqlite3
    import json
    
    def upload_to_sqlite(
        db_filename, table_name, source_file, columns,
    ):
        columns = json.loads(columns)

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
                type_fn += [lambda x: x]
            elif v == 'INTEGER':
                type_fn += [int]
            elif v == 'REAL':
                type_fn += [float]
            elif v == 'BLOB':
                type_fn += [bytes]
            else:
                type_fn += [lambda _: None]

        attr_names = [x[0] for x in columns]
        with open(source_file, 'r') as f:
            for line in f.readlines():
                x = json.loads(line)
                obj = [x[name] if name in x else None for name in attr_names]
                assert len(obj) == len(type_fn)
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
        print("Example record:\n" + "\n".join(["%s: %s" % x for x in zip(attr_names, row)]))
        con.close()
