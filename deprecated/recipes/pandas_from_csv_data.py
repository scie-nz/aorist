from aorist import aorist, PandasFromCSVData

programs = {}

@aorist(
    programs,
    PandasFromCSVData,
    entrypoint="read_csv_into_pandas",
    args={
        "dataset_name": lambda lng: lng.data_set.name,
        "static_data_table_name": lambda lng: lng.static_data_table.name,
        "columns": lambda lng: lng.serde_json::json!(static_data_table.schema.get_attribute_names()),
        "tmp_dir": lambda lng: lng.replication_storage_setup.tmp_dir,
    },
)
def recipe(dataset_name, static_data_table_name, columns, tmp_dir):
    
    import pandas as pd
    import json
    
    def read_csv_into_pandas(
        static_data_table_name,
        dataset_name,
        columns,
        tmp_dir,
    ):
        columns = json.loads(columns)
        data = pd.read_csv(
            "{tmp_dir}/{dataset_name}/{static_data_table_name}/{static_data_table_name}.csv".format(
                tmp_dir=tmp_dir,
                dataset_name=dataset_name,
                static_data_table_name=static_data_table_name,
            ),
            header=None
        )
        data.columns = columns
        print("Downloaded dataframe with %d rows and %d columns." % data.shape)
        return data
    
    