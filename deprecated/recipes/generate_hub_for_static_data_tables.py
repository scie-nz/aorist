from aorist import aorist, GenerateHubForStaticDataTables

programs = {}

@aorist(
    programs,
    GenerateHubForStaticDataTables,
    entrypoint="gen_hub_for_static_data_table",
    args={
        "dataset_name": lambda lng: lng.data_set.name,
        "code": lambda lng: lng.{
  use crate::layout::*;
  use crate::location::*;
  use crate::encoding::*;
  use crate::storage_setup::*;
  use crate::storage::*;

  let layout = FileBasedStorageLayout::SingleFileLayout(SingleFileLayout{
      constraints: Vec::new(), tag: None, uuid: None,
  });
  let encoding = match static_data_table.setup {
    StorageSetup::RemoteStorageSetup(ref s) => match s.remote {
          Storage::RemoteStorage(ref r) => match r.location {
              RemoteLocation::PushshiftAPILocation(_) => Encoding::NewlineDelimitedJSONEncoding(NewlineDelimitedJSONEncoding{
                  constraints: Vec::new(), tag: None, uuid: None,
              }),
              _ => Encoding::CSVEncoding(CSVEncoding{
                  compression: None,
                  header: None,
                  constraints: Vec::new(), tag: None, uuid: None,
              })
          },
          Storage::GitStorage(ref g) => g.encoding,
          _ => panic!("Only GitStorage or RemoteStorage supported"),
      },
      _ => panic!("Only RemoteStorageSetup supported"),
  };
  let tmp_dir = format!(
      "/tmp/{}/{}",
      data_set.name,
      static_data_table.name
  );
  let location = OnPremiseLocation::LocalFileSystemLocation(LocalFileSystemLocation{
      path: format!("{}/data.csv", tmp_dir),
      constraints: Vec::new(), tag: None, uuid: None,
  });
  let storage = Storage::LocalFileStorage(LocalFileStorage {
      location,
      layout,
      encoding: encoding,
      constraints: Vec::new(), tag: None, uuid: None,
  });
  let replicated_table = Asset::StaticDataTable(
      static_data_table.replicate_to_local(storage, tmp_dir, encoding)
  );
  let template = data_set.get_template_for_asset(static_data_table);
  let new_dataset = DataSet{
      name: data_set.name,
      description: data_set.description,
      sourcePath: data_set.sourcePath,
      accessPolicies: Vec::new(),
      datumTemplates: vec![template],
      assets: vec![(static_data_table.name, replicated_table)].into_iter().collect(),
      constraints: Vec::new(), tag: None, uuid: None,
  };
  let mut new_universe = Universe{
      name: universe.name,
      users: None,
      groups: None,
      datasets: Some(vec![new_dataset]),
      role_bindings: None,
      endpoints: universe.endpoints,
      compliance: None,
      models: None,
      constraints: Vec::new(), tag: None, uuid: None,
  };
  new_universe.compute_uuids();
  let (pandas_code, pandas_requirements) = PythonBasedDriver::<PythonFlowBuilder>::new(
      &new_universe,
      vec!["PandasData"].into_iter().collect(),
  ).run();
  let (numpy_code, numpy_requirements) = PythonBasedDriver::<PythonFlowBuilder>::new(
      &new_universe,
      vec!["NumpyData"].into_iter().collect(),
  ).run();
  let (r_code, r_requirements) = RBasedDriver::<RBasedFlowBuilder>::new(
      &new_universe,
      vec!["RDataFrame"].into_iter().collect(),
  ).run();
  serde_json::json!(maplit::hashmap!(
      "pandas_code" => serde_json::json!(
          pandas_code.replace("\\\\", "\\")
      ),
      "pandas_requirements" => serde_json::json!(pandas_requirements),
      "numpy_code" => serde_json::json!(
          numpy_code.replace("\\\\", "\\")
      ),
      "numpy_requirements" => serde_json::json!(numpy_requirements),
      "r_code" => serde_json::json!(r_code.replace("\\\\", "\\")),
      "r_requirements" => serde_json::json!(r_requirements),
  )).replace("\\", "\\\\")
}
,
        "static_data_table_name": lambda lng: lng.static_data_table.name,
        "storage_location_name": lambda lng: lng.{
  if let crate::StorageSetup::RemoteStorageSetup(rss) = &static_data_table.setup {
    match &rss.remote {
      crate::Storage::RemoteStorage(rs) => {
        match &rs.location {
          RemoteLocation::GCSLocation(_) => "GCSLocation",
          RemoteLocation::GithubLocation(_) => "GithubLocation",
          RemoteLocation::WebLocation(_) => "WebLocation",
          RemoteLocation::PushshiftAPILocation(_) => "PushshiftAPILocation",
          RemoteLocation::BigQueryLocation(_) => "BigQueryLocation",
        }
      },
      crate::Storage::GitStorage(_) => "GithubLocation",
      _ => panic!("Only RemoteStorage or GitStorage supported"),
    }
  } else {
    "OtherStorageSetup"
  }
}
,
        "storage_location_params": lambda lng: lng.{
  let mut params = std::collections::HashMap::new();
  if let crate::StorageSetup::RemoteStorageSetup(rss) = &static_data_table.setup {
    if let crate::Storage::RemoteStorage(rs) = &rss.remote {
      match &rs.location {
        RemoteLocation::GCSLocation(gcs) => {
          params.insert("bucket", gcs.bucket);
          params.insert("blob", gcs.blob);
        }
        RemoteLocation::GithubLocation(github) => {
          params.insert("organization", github.organization);
          params.insert("repository", github.repository);
          params.insert("path", github.path);
        }
        RemoteLocation::WebLocation(web) => {
          params.insert("address", web.address);
        },
        RemoteLocation::PushshiftAPILocation(papi) => {
          params.insert("subreddit", papi.subreddit);
        },
        RemoteLocation::BigQueryLocation(_bq) => {},
      };
    }
  }
  serde_json::json!(params)
}
,
        "attributes": lambda lng: lng.{
  let template = data_set.get_template_for_asset(static_data_table);
  serde_json::json!(
      template.get_attributes().iter().map(|x| (x.get_name(), x.get_type())).collect::<Vec<_>>()
  )
}
,
        "template_name": lambda lng: lng.static_data_table.get_schema().get_datum_template_name(),
    },
)
def recipe(dataset_name, code, static_data_table_name, storage_location_name, storage_location_params, attributes, template_name):
    from datetime import datetime
    import json
    import os
    import yaml
    
    def gen_hub_for_static_data_table(
        static_data_table_name,
        attributes,
        template_name,
        storage_location_name,
        storage_location_params,
        code,
        dataset_name,
    ):
        payload = {
            "name": static_data_table_name,
            "dataset_name": dataset_name,
            "template_name": template_name,
            "render_date": datetime.today().strftime('%Y-%m-%d'),
            "storage_location_name": storage_location_name,
            "storage_location_params": json.loads(storage_location_params),
            "attributes": json.loads(attributes),
            "code": json.loads(code),
        }
    
        base_dir = os.getenv('OUTPUT_DIR')
        if base_dir is None:
            raise Exception('Missing environment variable: OUTPUT_DIR')
        output_dir = os.path.join(base_dir, dataset_name)
    
        if not os.path.exists(output_dir):
            os.makedirs(output_dir)
        with open(os.path.join(output_dir, static_data_table_name + '.yaml'), 'w') as f:
            yaml.dump(payload, f, indent=2)
    
    