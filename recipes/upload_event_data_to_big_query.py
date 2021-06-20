from aorist import aorist, UploadEventDataToBigQuery

programs = {}

@aorist(
    programs,
    UploadEventDataToBigQuery,
    entrypoint="upload_events_to_bigquery",
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
        "source_file": lambda lng: lng.format!("{}.csv", static_data_table.name),
        "columns": lambda lng: lng.{
  use crate::template::TDatumTemplate;
  let template =
  data_set.get_template_for_asset(static_data_table);
  let attributes = template.get_attributes();
  let attributes_vec = attributes.into_iter().map(
      |x| (
          x.get_name(),
          x.get_bigquery_type(),
          x.is_nullable(),
      )
  ).collect::<Vec<_>>();
  serde_json::json!(attributes_vec)
}
,
        "time_ordering_columns": lambda lng: lng.{
    if let DataSchema::TimeOrderedTabularSchema(ref s) =
    static_data_table.schema {
        serde_json::json!(s.orderingAttributes)
    } else {
        panic!("Schema should be TimeOrderedTabularSchema.");
    }
}
,
        "source_is_json": lambda lng: lng.format!("{}", match storage_setup {
  crate::StorageSetup::ReplicationStorageSetup(r) =>
    match r.source {
       Storage::PostgresStorage(_) => "True",
       _ => match r.source.get_encoding() {
          Some(crate::Encoding::JSONEncoding(_)) => "True",
          _ => "False",
      },
    },
    _ => "False",
})
,
    },
)
def recipe(project_name, dataset_location, service_account_file, dataset_name, table_name, tmp_dir, source_file, columns, time_ordering_columns, source_is_json):
    def upload_events_to_bigquery(
        project_name,
        dataset_name,
        dataset_location,
        table_name,
        tmp_dir,
        source_file,
        service_account_file,
        source_is_json,
        columns,
        time_ordering_columns,
    ):
        import json
        from google.cloud import bigquery
        from google.oauth2 import service_account
        from google.cloud.exceptions import NotFound
        from google.api_core.exceptions import BadRequest
    
        if service_account_file != "":
            credentials = service_account.Credentials.from_service_account_file(service_account_file)
            client = bigquery.Client(credentials=credentials)
        else:
            client = bigquery.Client()
    
        source_is_json = bool(source_is_json)
    
        dataset_id = project_name + "." + dataset_name
        try:
            dataset = client.get_dataset(dataset_id)
        except NotFound:
            dataset = bigquery.Dataset(dataset_id)
            dataset.location = dataset_location
            client.create_dataset(dataset)
            dataset = client.get_dataset(dataset_id)
    
    
        table_id = "{project_name}.{dataset_name}.{table_name}".format(
            project_name=project_name,
            dataset_name=dataset_name,
            table_name=table_name,
        )
        try:
            table = client.get_table(table_id)
        except NotFound:
            table_ref = dataset.table(table_name)
            columns = json.loads(columns)
            schema = [
                bigquery.SchemaField(k, v, mode="REQUIRED" if not n else "NULLABLE")
                for k, v, n in columns
            ]
            table = bigquery.Table(table_ref, schema=schema)
            client.create_table(table)
    
        job_config = bigquery.LoadJobConfig(
            source_format=bigquery.SourceFormat.NEWLINE_DELIMITED_JSON,
            ignore_unknown_values=True,
        ) if source_is_json else bigquery.LoadJobConfig(
            source_format=bigquery.SourceFormat.CSV,
            skip_leading_rows=0,
        )
    
        with open(tmp_dir + '/' + source_file, 'rb') as f:
            job = client.load_table_from_file(f, table_id, job_config=job_config)
    
        try:
            job.result()  # Waits for the job to complete.
        except BadRequest:
            print("Errors occurred while loading JSON:")
            print(job.errors)
    
        table = client.get_table(table_id)  # Make an API request.
    
        print(
            "Table {} now has {} rows and {} columns.".format(
                table_id, table.num_rows, len(table.schema),
            )
        )
    
    