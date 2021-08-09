from aorist import aorist, PandasFromNewlineDelimitedJSONData

programs = {}

@aorist(
    programs,
    PandasFromNewlineDelimitedJSONData,
    entrypoint="read_json_into_pandas",
    args={
        "dataset_name": lambda lng: lng.data_set.name,
        "static_data_table_name": lambda lng: lng.static_data_table.name,
        "columns": lambda lng: lng.serde_json::json!(static_data_table.schema.get_attribute_names()),
    },
)
def recipe(dataset_name, static_data_table_name, columns):
    
    import pandas as pd
    import json
    
    def read_json_into_pandas(
        static_data_table_name,
        dataset_name,
        columns,
    ):
        columns = json.loads(columns)
    
        with open(
            "/tmp/{dataset_name}/{static_data_table_name}/data.json".format(
                dataset_name=dataset_name, static_data_table_name=static_data_table_name
            )
        ) as f:
            raw_records = [json.loads(l) for l in f.readlines()]
    
        records = [[r[k] for k in columns] for r in raw_records]
        return pd.DataFrame(records, columns=columns)
    
    