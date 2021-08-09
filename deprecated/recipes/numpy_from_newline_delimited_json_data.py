from aorist import aorist, NumpyFromNewlineDelimitedJSONData

programs = {}

@aorist(
    programs,
    NumpyFromNewlineDelimitedJSONData,
    entrypoint="read_json_into_numpy",
    args={
        "dataset_name": lambda lng: lng.data_set.name,
        "static_data_table_name": lambda lng: lng.static_data_table.name,
        "columns": lambda lng: lng.serde_json::json!(static_data_table.schema.get_attribute_names()),
    },
)
def recipe(dataset_name, static_data_table_name, columns):
    
    import numpy as np
    import json
    
    def read_json_into_numpy(
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
        records = [[str(r[k]) for k in columns] for r in raw_records]
        max_str_lens = [max([len(y) for y in x]) for x in zip(*records)]
        ndarray = np.array(
            records,
            dtype=[
                (x, "U%d" % l)
                for l, x in
                zip(max_str_lens, columns)
            ]
        )
        return ndarray
    
    