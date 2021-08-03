/***
@aorist_presto(
    programs,
    ORCTableSchemasCreated,
    args={
        "presto_schema": lambda data_set: data_set.name,
        "table_name": lambda static_data_table: static_data_table.name,
        "columns": lambda data_set, asset: ",\n".join([
            "{name} {presto_type}{comment}".format(
                name=x.name,
                presto_type=x.presto_type,
                comment=(
                    "COMMENT '%s'" % x.comment.replace("'", "`")
                ) if x.comment is not None else "",
            ) for x in data_set.get_template(asset).attributes()
        ]),
    },
)
***/
CREATE TABLE {presto_schema}.{table_name} (
    {columns}
)
