from aorist import aorist, NumpyFromCSVData

programs = {}

@aorist(
    programs,
    NumpyFromCSVData,
    entrypoint="read_csv_into_numpy",
    args={
        "dataset_name": lambda lng: lng.data_set.name,
        "static_data_table_name": lambda lng: lng.static_data_table.name,
        "columns": lambda lng: lng.serde_json::json!(static_data_table.schema.get_attribute_names()),
        "tmp_dir": lambda lng: lng.replication_storage_setup.tmp_dir,
    },
)
def recipe(dataset_name, static_data_table_name, columns, tmp_dir):
    
    import numpy as np
    import json
    import csv
    
    def read_csv_into_numpy(static_data_table_name, dataset_name, columns, tmp_dir):
        columns = json.loads(columns)
        file_name = (
            "{tmp_dir}/{dataset_name}/{static_data_table_name}"
            "/{static_data_table_name}.csv"
        ).format(
            dataset_name=dataset_name,
            static_data_table_name=static_data_table_name,
            tmp_dir=tmp_dir,
        )
        escaped = ("\t".join(
            [i.replace("\t", " ") for i in x]
        ) for x in csv.reader(open(file_name)))
        data = np.genfromtxt(
            escaped,
            delimiter="\t",
            names=columns,
            dtype=None,
        )
        assert data is not None
        print("Downloaded ndarray with %d rows." % data.shape[0])
        return data
    
    