from aorist import aorist, DetermineMaxBigQueryTimeStamp

programs = {}

@aorist(
    programs,
    DetermineMaxBigQueryTimeStamp,
    entrypoint="determine_max_bigquery_timestamp",
    args={
        "project_name": lambda lng: lng.universe.endpoints.gcp.project_name,
        "dataset_location": lambda lng: lng.universe.endpoints.gcp.data_location,
        "service_account_file": lambda lng: lng.match universe.endpoints.gcp.service_account_file {
  Some(ref f) => f,
  None => "",
}
,
        "dataset_name": lambda lng: lng.data_set.name,
        "table_name": lambda lng: lng.static_data_table.name,
        "tmp_dir": lambda lng: lng.replication_storage_setup.tmp_dir,
        "time_ordering_columns": lambda lng: lng.{
    if let DataSchema::TimeOrderedTabularSchema(ref s) =
    static_data_table.schema {
        serde_json::json!(s.orderingAttributes)
    } else {
        panic!("Schema should be TimeOrderedTabularSchema.");
    }
}
,
    },
)
def recipe(project_name, dataset_location, service_account_file, dataset_name, table_name, tmp_dir, time_ordering_columns):
    import json
    from google.cloud import bigquery
    from google.oauth2 import service_account
    from google.cloud.exceptions import NotFound
    from google.api_core.exceptions import BadRequest
    
    def determine_max_bigquery_timestamp(
        project_name,
        dataset_name,
        dataset_location,
        table_name,
        tmp_dir,
        service_account_file,
        time_ordering_columns,
    ):
        if not os.path.exists(tmp_dir):
            os.makedirs(tmp_dir)
        if service_account_file != "":
            credentials = service_account.Credentials.from_service_account_file(service_account_file)
            client = bigquery.Client(credentials=credentials)
        else:
            client = bigquery.Client()
    
        dataset_id = project_name + "." + dataset_name
        try:
            dataset = client.get_dataset(dataset_id)
        except NotFound:
            return
    
        table_id = "{project_name}.{dataset_name}.{table_name}".format(
            project_name=project_name,
            dataset_name=dataset_name,
            table_name=table_name,
        )
        try:
            table = client.get_table(table_id)
        except NotFound:
            return
    
        time_ordering_columns = json.loads(time_ordering_columns)
        col = time_ordering_columns[0]
        query = """
        SELECT MAX({col}) AS max_val
        FROM {table_id}
        """.format(table_id=table_id, col=col)
    
        # TODO: support multiple time ordering columns
        assert len(time_ordering_columns) == 1
    
        query_job = client.query(query)
    
    
        try:
            results = query_job.result()  # Waits for the job to complete.
        except BadRequest:
            print("Errors occurred while loading JSON:")
            print(job.errors)
    
        max_val = list(results)[0].max_val
    
        table = client.get_table(table_id)  # Make an API request.
    
        print(
            "Max. value in BQ for column {col} is {max_val}".format(
                col=col,
                max_val=max_val,
            )
        )
        obj = { col: max_val }
        with open(tmp_dir + '/' + table_name + '.bqmax', 'w') as f:
            json.dump(obj, f)
    
    