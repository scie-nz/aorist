from aorist import aorist, UploadDataToPostgres

programs = {}

@aorist(
    programs,
    UploadDataToPostgres,
    entrypoint="upload_to_postgres",
    args={
        "db_name": lambda lng: lng.data_set.name,
        "server": lambda lng: lng.universe.endpoints.postgres.server,
        "port": lambda lng: lng.universe.endpoints.postgres.port,
        "username": lambda lng: lng.universe.endpoints.postgres.username,
        "password": lambda lng: lng.universe.endpoints.postgres.password,
        "tablename": lambda lng: lng.static_data_table.name,
        "tmp_dir": lambda lng: lng.replication_storage_setup.tmp_dir,
        "source_file": lambda lng: lng.format!("{}.csv", static_data_table.name),
        "columns": lambda lng: lng.{
  use crate::template::TDatumTemplate;
  let template =
  data_set.get_template_for_asset(static_data_table);
  let attributes = template.get_attributes();
  let attributes_vec = attributes.into_iter().map(
      |x| (
          x.get_name(),
          x.get_postgres_type(),
          x.is_nullable(),
      )
  ).collect::<Vec<_>>();
  serde_json::json!(attributes_vec)
}
,
        "source_is_json": lambda lng: lng.format!("{}", match storage_setup {
  crate::StorageSetup::ReplicationStorageSetup(r) =>
    match r.source.get_encoding() {
      Some(crate::Encoding::JSONEncoding(_)) => "True",
      _ => "False",
    },
    _ => "False",
})
,
    },
)
def recipe(db_name, server, port, username, password, tablename, tmp_dir, source_file, columns, source_is_json):
    def upload_to_postgres(
        server,
        port,
        username,
        password,
        db_name,
        tablename,
        tmp_dir,
        source_file,
        # insertion order matters if we are dealing with a csv
        columns,
        source_is_json,
    ):
        import psycopg2
        import json
        from psycopg2.extras import execute_batch
    
        source_is_json = bool(source_is_json)
    
        conn_string = " ".join([
            "host=%s" % server,
            "port=%s" % port,
            "dbname=%s" % db_name,
            "user=%s" % username,
            "password=%s" % password,
        ])
        conn=psycopg2.connect(conn_string)
        cursor = conn.cursor()
    
        columns = json.loads(columns)
        cursor.execute("""
        DROP TABLE IF EXISTS {tablename}
        """.format(
            tablename=tablename,
        ))
        cursor.execute("""
        CREATE TABLE {tablename}({schema})
        """.format(
            tablename=tablename,
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
            if source_is_json:
                for line in f.readlines():
                    obj = json.loads(line)
                    tpl = tuple(
                        fn(obj[x])
                        if not nullable
                        else (
                            lambda y: fn(obj[y]) if y in obj else None
                        )(x)
                        for fn, x, nullable in zip(
                            type_fn,
                            [k for k, _, _ in columns],
                            [n for _k, _v, n in columns]
                        )
                    )
                    values += [tpl]
            else:
                for line in f.readlines():
                    splits = line.strip().split(",")
                    assert len(splits) == len(type_fn)
                    tpl = tuple(fn(arg) for fn, arg in zip(type_fn, splits))
                    values += [tpl]
    
        execute_batch(
            cursor,
            """
                INSERT INTO {tablename}({columns}) VALUES ({vals})
            """.format(
                tablename=tablename,
                columns=", ".join([k for k, _, _ in columns]),
                vals=", ".join(["%s"] * len(columns)),
            ),
            values,
        )
        conn.commit()
        cursor.execute("SELECT COUNT(*) FROM " + tablename)
        conn.commit()
        count = cursor.fetchall()[0][0]
        print("Inserted %d records into %s" % (count, tablename))
    
    