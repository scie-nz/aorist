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
        "tmp_dir": lambda replication_storage_setup: replication_storage_setup.tmp_dir,
        "source_file": lambda static_data_table: static_data_table.name + ".csv",
        "columns": lambda data_set, asset: dumps([
            (x.name, x.sqlite_type, x.is_nullable)
            for x in data_set.get_template(asset).attributes()
        ]),
    }
)
def recipe(
    db_filename, table_name, tmp_dir, source_file,
    # insertion order matters if we are dealing with a csv
    columns,
):
    
    import sqlite3
    import json
    
    def upload_to_sqlite(
        db_filename, table_name, tmp_dir, source_file,
        # insertion order matters if we are dealing with a csv
        columns,
    ):
        columns = json.loads(columns)

        con = sqlite3.connect(db_filename)
        con.execute("""
        DROP TABLE IF EXISTS {table_name}
        """.format(
            table_name=table_name,
        ))
        con.execute("""
        CREATE TABLE {table_name}({schema})
        """.format(
            table_name=table_name,
            schema=",\\n".join(["%s %s" % (k, v) for k, v, _ in columns]),
        ))

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

        with open(tmp_dir + '/' + source_file, 'r') as f:
            for line in f.readlines():
                splits = line.strip().split(",")
                assert len(splits) == len(type_fn)
                tpl = tuple(fn(arg) for fn, arg in zip(type_fn, splits))
                values += [tpl]

        con.executemany(
            """
                INSERT INTO {table_name}({columns}) VALUES ({vals})
            """.format(
                table_name=table_name,
                columns=", ".join([k for k, _, _ in columns]),
                vals=", ".join(["?"] * len(columns))
            ),
            values
        )
        con.commit()
        count = list(con.execute("SELECT COUNT(*) FROM " + table_name))[0][0]
        print("Inserted %d records into %s" % (count, table_name))
        con.close()
