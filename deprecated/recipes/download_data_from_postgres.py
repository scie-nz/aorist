from aorist import aorist, DownloadDataFromPostgres

programs = {}

@aorist(
    programs,
    DownloadDataFromPostgres,
    entrypoint="download_from_postgres",
    args={
        "db_name": lambda lng: lng.postgres_location.database,
        "schema_name": lambda lng: lng.postgres_location.schema,
        "server": lambda lng: lng.universe.endpoints.postgres.server,
        "port": lambda lng: lng.universe.endpoints.postgres.port,
        "username": lambda lng: lng.universe.endpoints.postgres.username,
        "password": lambda lng: lng.universe.endpoints.postgres.password,
        "tablename": lambda lng: lng.static_data_table.name,
        "tmp_dir": lambda lng: lng.replication_storage_setup.tmp_dir,
        "columns": lambda lng: lng.{
  use crate::template::TDatumTemplate;
  let template =
  data_set.get_template_for_asset(static_data_table);
  let attributes = template.get_attributes();
  let attributes_vec = attributes.into_iter().map(
      |x| (x.get_name(), x.psycopg2_value_json_serializable())
  ).collect::<Vec<_>>();
  serde_json::json!(attributes_vec)
}
,
        "time_ordering_columns": lambda lng: lng.{
    if let DataSchema::TimeOrderedTabularSchema(ref s) =
    static_data_table.schema {
        serde_json::json!(s.orderingAttributes)
    } else {
        "[]"
    }
}
,
    },
)
def recipe(db_name, schema_name, server, port, username, password, tablename, tmp_dir, columns, time_ordering_columns):
    import base64
    import json
    import psycopg2
    import os
    from dateutil.parser import parse
    from datetime import datetime, timezone
    
    def download_from_postgres(
        server,
        port,
        username,
        password,
        db_name,
        tablename,
        tmp_dir,
        columns,
        schema_name,
        time_ordering_columns,
    ):
    
        # assert len(time_ordering_columns) <= 1
        from collections import deque
    
        if not os.path.exists(tmp_dir):
            os.makedirs(tmp_dir)
        time_ordering_columns = json.loads(time_ordering_columns)
    
        conn_string = " ".join(
            [
                "host=%s" % server,
                "port=%s" % port,
                "dbname=%s" % db_name,
                "user=%s" % username,
                "password=%s" % password,
            ]
        )
        conn = psycopg2.connect(conn_string)
        cursor = conn.cursor()
        columns = json.loads(columns)
        query = """
        SELECT {columns} FROM {db_name}.{schema_name}."{tablename}"
        """.format(
            columns=", ".join(
                [
                    ('"%s"' % x[0] if x[1] else '"%s"::text AS %s' % (x[0], x[0]))
                    for x in columns
                ]
            ),
            tablename=tablename,
            db_name=db_name,
            schema_name=schema_name,
        )
        with open(os.path.join(tmp_dir, tablename + ".csv"), "w") as f:
            if len(time_ordering_columns) == 1:
                col = time_ordering_columns[0]
                try:
                    with open(tmp_dir + "/" + tablename + ".bqmax", "r") as fbq:
                        bqmax = json.load(fbq)
                except FileNotFoundError:
                    bqmax = None
                fmt = "%Y-%m-%d %H:%M:%S%z"
                dq = deque([
                    (
                        bqmax[col] if bqmax is not None else None,
                        datetime.strftime(datetime.now(timezone.utc), fmt)
                    )
                ])
    
                while len(dq) > 0:
                    (start_ts, end_ts) = dq.pop()
                    print(
                        "Downloading data for table %s with time column %s between %s and %s" % (
                            tablename,
                            col,
                            start_ts if start_ts is not None else "beginning of time",
                            end_ts,
                        )
                    )
                    max_clause = (
                        """
                        WHERE "{col}" > '{start_ts}'::timestamptz
                        AND "{col}" <= '{end_ts}'::timestamptz
                        """.format(col=col, start_ts=start_ts, end_ts=end_ts)
                        if start_ts is not None else
                        """
                        WHERE "{col}" <= '{end_ts}'::timestamptz
                        """.format(col=col, end_ts=end_ts)
                    )
                    interval_query = """
                    {query}
                    {max_clause}
                    """.format(
                        query=query,
                        max_clause=max_clause
                    )
                    try:
                        print(interval_query)
                        cursor = conn.cursor()
                        cursor.execute(interval_query)
                        keys = [x[0] for x in columns]
                        num_rows = 0
                        for row in cursor.fetchall():
                            row = [
                                (
                                    x
                                    if not isinstance(x, (bytes, bytearray, memoryview))
                                    else base64.b64encode(x).decode("ascii")
                                )
                                for x in row
                            ]
                            map = dict(zip(keys, row))
                            f.write(("%s" % json.dumps(map)) + '\\n')
                            num_rows += 1
                        print("Downloaded %d rows from table %s" % (num_rows, tablename))
                    except psycopg2.errors.SerializationFailure:
                        try:
                            conn.rollback()
                        except psycopg2.InterfaceError:
                            conn = psycopg2.connect(conn_string)
    
                        print("Downloading data took too long for this chunk, splitting into two.")
                        x = parse(start_ts)
                        y = parse(end_ts)
                        midpoint = x + (y - x) / 2
                        midpoint = datetime.strftime(midpoint, fmt)
                        dq.append((midpoint, end_ts))
                        dq.append((start_ts, midpoint))
    
            else:
                cursor = conn.cursor()
                cursor.execute(query)
                keys = [x[0] for x in columns]
                num_rows = 0
                for row in cursor.fetchall():
                    row = [
                        (
                            x
                            if not isinstance(x, (bytes, bytearray, memoryview))
                            else base64.b64encode(x).decode("ascii")
                        )
                        for x in row
                    ]
                    map = dict(zip(keys, row))
                    f.write(("%s" % json.dumps(map)) + '\\n')
                    num_rows += 1
                print("Downloaded %d rows from table %s" % (num_rows, tablename))
    
    