/***
@aorist_presto(
    programs,
    ConvertJSONTableToORCTable,
    args={
        "presto_schema": lambda data_set: data_set.name,
        "source_table": lambda static_data_table: "tmp_" + static_data_table.name,
        "table_name": lambda static_data_table: static_data_table.name,
        "columns": lambda data_set, asset: ", ".join([
            x.name for x in data_set.get_template(asset).attributes()
        ]),
    },
)
***/
INSERT INTO {presto_schema}.{table_name}
SELECT {columns}
FROM {source_table}
