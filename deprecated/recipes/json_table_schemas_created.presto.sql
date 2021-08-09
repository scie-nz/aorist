/***
@aorist_presto(
    programs,
    JSONTableSchemasCreated,
    args={
        "presto_schema": lambda data_set: data_set.name,
        "table_name": lambda asset: asset.name(),
    },
)
***/
CREATE TABLE IF NOT EXISTS {presto_schema}.{table_name} (
    json_obj VARCHAR
)
WITH (format='CSV')
