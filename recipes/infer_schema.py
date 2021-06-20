from aorist import aorist, InferSchema

programs = {}

@aorist(
    programs,
    InferSchema,
    entrypoint="infer_schema_from_postgres",
    args={
        "db_name": lambda lng: lng.data_set.name,
        "dataset_description": lambda lng: lng.data_set.description,
        "server": lambda lng: lng.universe.endpoints.postgres.server,
        "port": lambda lng: lng.universe.endpoints.postgres.port,
        "username": lambda lng: lng.universe.endpoints.postgres.username,
        "password": lambda lng: lng.universe.endpoints.postgres.password,
    },
)
def recipe(db_name, dataset_description, server, port, username, password):
    import json
    
    def infer_schema_from_postgres(
        server,
        port,
        username,
        password,
        db_name,
        dataset_description,
    ):
        import psycopg2
        import black
    
        conn_string = " ".join([
            "host=%s" % server,
            "port=%s" % port,
            "dbname=%s" % db_name,
            "user=%s" % username,
            "password=%s" % password,
        ])
        conn=psycopg2.connect(conn_string)
        cursor = conn.cursor()
        cursor.execute("""
        SELECT table_name, table_schema
        FROM information_schema.tables
        """)
        coldefs = []
    
        attr_types = {
            "BOOL": "attr.FromPostgresBool",
            "TEXT": "attr.FromPostgresText",
            "UUID": "attr.FromPostgresUuid",
            "OID": "attr.FromPostgresOid",
            "NAME": "attr.FromPostgresName",
            "INTEGER": "attr.FromPostgresInteger",
            "BIGINT": "attr.FromPostgresBigInt",
            "CHARACTER VARYING": "attr.FromPostgresCharacterVarying",
            "TIMESTAMP WITHOUT TIME ZONE": "attr.FromPostgresTimestampWithoutTimeZone",
            "TIMESTAMP WITH TIME ZONE": "attr.FromPostgresTimestampWithTimeZone",
            "CHAR": "attr.FromPostgresChar",
            "BOOLEAN": "attr.FromPostgresBoolean",
            "JSONB": "attr.FromPostgresJSONB",
            "USER-DEFINED": "attr.FromPostgresUserDefined",
            "ARRAY": "attr.FromPostgresArray",
            "REGPROC": "attr.FromPostgresRegProc",
            "SMALLINT": "attr.FromPostgresSmallInt",
            "REAL": "attr.FromPostgresReal",
            "DOUBLE PRECISION": "attr.FromPostgresDoublePrecision",
            "PG_NODE_TREE": "attr.FromPostgresPgNodeTree",
            "PG_LSN": "attr.FromPostgresPgLsn",
            "XID": "attr.FromPostgresXid",
            "INTERVAL": "attr.FromPostgresInterval",
            "ANYARRAY": "attr.FromPostgresAnyArray",
            "BYTEA": "attr.FromPostgresBytea",
            "REGTYPE": "attr.FromPostgresRegType",
            "PG_NDISTINCT": "attr.FromPostgresPgNDistinct",
            "PG_DEPENDENCIES": "attr.FromPostgresPgDependencies",
            "INET": "attr.FromPostgresInet",
            "NUMERIC": "attr.FromPostgresNumeric",
        }
        time_dimensions = {
            "TIMESTAMP WITHOUT TIME ZONE",
            "TIMESTAMP WITH TIME ZONE",
        }
        templates = {}
        assets = {}
    
        for table_name, table_schema in cursor.fetchall():
            cursor.execute(
                """
                SELECT column_name, data_type, is_nullable
                FROM information_schema.columns
                WHERE table_name = %s AND table_schema = %s
                AND NOT table_name LIKE 'information_schema%%'
                AND table_schema != 'information_schema'
                AND table_schema != 'pg_catalog'
                """,
                (table_name, table_schema),
            )
            template = []
            has_time_dimension = False
            time_dimension_columns = []
            maybe_time_dimension_columns = []
            for column_name, data_type, is_nullable in cursor.fetchall():
                is_nullable = False if is_nullable == "NO" else True
                is_time_dimension = (
                    data_type.upper() in time_dimensions
                )
                if is_time_dimension and not has_time_dimension:
                    if is_nullable:
                        print(maybe_time_dimension_columns)
                        maybe_time_dimension_columns += [column_name]
                    else:
                        has_time_dimension = True
                        time_dimension_columns += [column_name]
                data_type = data_type.upper().replace('"', "")
                if not data_type in attr_types:
                    print("Problem with attr:" + data_type)
                else:
                    attr_type = attr_types[data_type]
                    template += [
                        "%s('%s'%s)"
                        % (
                            attr_type,
                            column_name,
                            ", nullable=True" if is_nullable else "",
                        )
                    ]
            if len(time_dimension_columns) == 0 and len(maybe_time_dimension_columns) > 0:
    
                print("Looking at potential time dimension columns for table %s" % table_name)
                query = "\\nUNION ALL\\n".join([
                    """
                    SELECT
                        '{col}' AS column,
                        COUNT(*) AS num_total,
                        COUNT(*) FILTER(WHERE "{col}" IS NULL) AS num_null
                    FROM {schema}.{table}
                    """.format(col=col, table=table_name, schema=table_schema)
                    for col in maybe_time_dimension_columns
                ])
                cursor.execute(query)
                res = cursor.fetchall()
                actually_time_dimension_columns = [x[0] for x in res if x[2] == 0]
                time_dimension_columns += actually_time_dimension_columns
                if len(time_dimension_columns) > 0:
                    print("Found time dimension column: %s" % time_dimension_columns[0])
                    has_time_dimension = True
    
            if len(template) > 0:
                template_tpl = tuple(template)
    
                if not template_tpl in templates:
                    templates[template_tpl] = table_name
                    template_name = table_name
                else:
                    template_name = templates[template_tpl]
    
                assets[table_name] = """
                StaticDataTable(
                    name="{table_name}",
                    schema={schema_call},
                    setup=RemoteStorageSetup(
                        remote=PostgresStorage(
                            location=PostgresLocation(
                                database="{db_name}",
                                schema="{table_schema}",
                            ),
                            layout=StaticTabularLayout(),
                        )
                    ),
                    tag="{table_name}",
                ),
                """.format(
                    table_name=table_name,
                    schema_call=(
                        (
                            "default_tabular_schema(templates['{template_name}'])"
                        ).format(template_name=template_name)
                        if not has_time_dimension else (
                            "default_time_ordered_tabular_schema("
                                "templates['{template_name}'], "
                                "{time_dimension_columns}"
                            ")"
                        ).format(
                            template_name=template_name,
                            time_dimension_columns=json.dumps(time_dimension_columns),
                        )
                    ),
                    db_name=db_name,
                    table_schema=table_schema
                )
    
        datum_templates = {}
        for template, name in templates.items():
            datum_templates[name] = (
                """
                RowStruct(
                    name="{name}",
                    attributes=attr_list([{attributes}])
                )
                """.format(
                    name=name,
                    attributes=", ".join(template),
                )
           )
    
        templates_code = (
              "templates = {{\\n{templates}\\n}}"
            ).format(
                templates="\\n".join([
                    '        "%s": %s,' % (k, v.strip())
                    for k, v in datum_templates.items()
                ]),
            )
        templates_code = black.format_str(templates_code, mode=black.Mode())
    
        assets_code = (
              "assets = {{\\n{assets}\\n}}"
            ).format(
                assets="\\n".join([
                    '        "%s": %s' % (k, v.strip())
                    for k, v in assets.items()
                ]),
            )
        assets_code = black.format_str(assets_code, mode=black.Mode())
    
        dataset_code = black.format_str("""
        {db_name} = DataSet(
            name="{db_name}",
            datumTemplates=list(templates.values()),
            assets=assets,
            description="{description}",
            sourcePath="",
        )
        """.format(
            db_name=db_name,
            description=dataset_description,
        ), mode=black.Mode())
    
        from IPython.display import Javascript, display
        import base64
    
        for code in [templates_code, assets_code, dataset_code]:
            encoded_code = (base64.b64encode(str.encode(code))).decode()
            display(Javascript("""
                var code = IPython.notebook.insert_cell_above('code');
                code.set_text(atob( "{0}" ));
                """.format(encoded_code)))
    
    