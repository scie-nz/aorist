from aorist import aorist, SVMRegressionModelsTrained

programs = {}

@aorist(
    programs,
    SVMRegressionModelsTrained,
    entrypoint="download_and_train",
    args={
        "minio_output_path": lambda lng: lng.{
  match supervised_model.setup {
      StorageSetup::ComputedFromLocalData(ref setup) => match
      setup.target {
          Storage::LocalFileStorage(ref storage) => match
          storage.location {
              OnPremiseLocation::MinioLocation(ref l) => format!(
                  "{}/{}.onnx",
                  l.name,
                  supervised_model.get_name(),
              ),
              _ => panic!("Only on-premise locations allowed."),
          },
          _ => panic!("Only local file storage allowed."),
      },
      _ => panic!("Only locally-computed models allowed."),
  }
}
,
        "objective": lambda lng: lng.{
  let template = data_set.get_template_for_asset(supervised_model);
  let tfm = match template {
      Ok(DatumTemplate::TrainedFloatMeasure(m)) => m,
      Err(x) => panic!("{}", x),
      _ => panic!(
          "Cannot train an SVM Regression Model on asset with with template {}",
          supervised_model.get_schema().get_datum_template_name(),
      ),
  };
  tfm.get_training_objective().get_name()
}
,
        "asset_name": lambda lng: lng.supervised_model.get_name(),
        "features": lambda lng: lng.{
  let template = data_set.get_template_for_asset(supervised_model);
  let tfm = match template {
      Ok(DatumTemplate::TrainedFloatMeasure(m)) => m,
      Err(x) => panic!("{}", x),
      _ => panic!(
          "Cannot train an SVM Regression Model on asset with with template {}",
          supervised_model.get_schema().get_datum_template_name(),
      ),
  };
  format!("[{}]",
      tfm.features.iter().map(|x| format!("\"{}\"", x.get_name())).collect::<Vec<String>>().join(","),
  )
}
,
        "minio": lambda lng: lng.{
  let endpoint_config = universe.endpoints;
  let minio = endpoint_config.minio;
  format!(
      "[\"{}\", {}, \"{}\", \"{}\", \"{}\"]",
      minio.server,
      minio.port,
      minio.access_key,
      minio.secret_key,
      minio.bucket,
  )
}
,
        "tmp_dir": lambda lng: lng.supervised_model.setup.get_tmp_dir(),
        "minio_prefix": lambda lng: lng.{
let computed_from_local_data = match &supervised_model.setup {
    StorageSetup::ComputedFromLocalData(m) => m,
    _ => panic!("Only ComputedFromLocalData StorageSetups allowed here."),
};
assert_eq!(computed_from_local_data.source_asset_names.len(), 1);
let source =
    computed_from_local_data.source_asset_names.values().next();
let source_asset = data_set.get_asset(source);
let storage_setup = source_asset.get_storage_setup();
let local = storage_setup.get_local_storage();
let minio = local
  .into_iter()
  .map(
    |x| if let Storage::HiveTableStorage(hive) = x {
      return Some(hive)
    } else {
      return None
    }
  ).filter(|x| x.is_some())
  .map(
    |x| if let HiveLocation::MinioLocation(m) = x.location {
      return Some(m);
    } else {
      return None;
    }
  )
  .filter(|x| x.is_some())
  .map(|x| x)
  .next()
  ;
  format!("{}/{}/", minio.name, source)
}
,
    },
)
def recipe(minio_output_path, objective, asset_name, features, minio, tmp_dir, minio_prefix):
    import pandas as pd
    import pyarrow.orc as orc
    from minio import Minio
    import json
    from skl2onnx import convert_sklearn
    from skl2onnx.common.data_types import FloatTensorType
    from sklearn.svm import SVR
    
    def download_and_train(
        minio,
        minio_prefix,
        tmp_dir,
        objective,
        features,
        asset_name,
        minio_output_path,
    ):
    
        (
            minio_hostname, minio_port, minio_access_key,
            minio_secret_key, minio_bucket
        ) = json.loads(minio)
    
    
        client = Minio(
            "%s:%s" % (minio_hostname, minio_port),
            access_key=minio_access_key,
            secret_key=minio_secret_key,
            secure=False,
        )
    
        assert client.bucket_exists(minio_bucket)
        objects = client.list_objects(minio_bucket, prefix=minio_prefix)
        datasets = []
        for i, obj in enumerate(objects):
            dest_file = '%s/%000d' % (tmp_dir, i)
            client.fget_object(
                minio_bucket,
                obj.object_name,
                file_path=dest_file,
            )
    
            with open(dest_file, 'rb') as f:
                data = orc.ORCFile(f)
                df = data.read().to_pandas()
                datasets += [df]
        data = pd.concat(datasets)
    
        features = json.loads(features)
        X = data[features].to_numpy()
        y = data[objective].to_numpy()
        svr = SVR(verbose=True)
        model = svr.fit(X, y)
    
        num_features = len(features)
        initial_type = [('float_input', FloatTensorType([None, num_features]))]
        onx = convert_sklearn(model, initial_types=initial_type)
    
        tmp_output_path = '%s/%s.onnx' % (tmp_dir, asset_name)
        with open(tmp_output_path, "wb") as f:
            f.write(onx.SerializeToString())
        client.fput_object(minio_bucket, minio_output_path, tmp_output_path)
    
    