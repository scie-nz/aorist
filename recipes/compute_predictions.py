from aorist import aorist, ComputePredictions

programs = {}

@aorist(
    programs,
    ComputePredictions,
    entrypoint="make_predictions",
    args={
        "asset_name": lambda lng: lng.computed_from_local_data.source_asset_names.values().next(),
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
        "tmp_dir": lambda lng: lng.computed_from_local_data.tmp_dir,
        "source_path": lambda lng: lng.{
  let template = data_set.get_template_for_asset(static_data_table);
  let ptfm = match template {
      Ok(DatumTemplate::PredictionsFromTrainedFloatMeasure(x)) => x,
      _ => panic!("Template must be of type PredictionsFromTrainedFloatMeasure."),
  };
  let source_asset_role = ptfm.get_source_asset_role();

  assert_eq!(computed_from_local_data.source_asset_names.len(), 2);
  let source_name =
      computed_from_local_data.source_asset_names.get(&source_asset_role);
  let source_asset = data_set.get_asset(source_name);

  match source_asset {
    Asset::StaticDataTable(ref static_data_table) => {
      let local = static_data_table.setup.get_local_storage();
      assert_eq!(local.len(), 1);
      let local_storage = local.iter().next();
      match local_storage {
            Storage::HiveTableStorage(ref storage) => match
            storage.location {
                HiveLocation::MinioLocation(ref l) =>
                  format!("{}/{}/", l.name, static_data_table.get_name()),
                _ => panic!("Only on-premise locations allowed."),
            },
            _ => panic!("Only HiveTableStorage allowed."),
      }
    }
    _ => panic!(
        "Source asset {} must be StaticDataTable", source_name,
    ),
  }
}
,
        "model_path": lambda lng: lng.{
  let template = data_set.get_template_for_asset(static_data_table);
  let ptfm = match template {
      Ok(DatumTemplate::PredictionsFromTrainedFloatMeasure(x)) => x,
      _ => panic!("Template must be of type PredictionsFromTrainedFloatMeasure."),
  };
  let model_asset_role = ptfm.get_model_asset_role();

  assert_eq!(computed_from_local_data.source_asset_names.len(), 2);
  let model_name =
      computed_from_local_data.source_asset_names.get(&model_asset_role);
  let source_asset = data_set.get_asset(model_name);

  match source_asset {
    Asset::SupervisedModel(ref supervised_model) =>
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
    },
    _ => panic!(
        "Model asset {} must be supervised model", model_name,
    ),
  }
}
,
        "destination_path": lambda lng: lng.{
  match static_data_table.setup {
      StorageSetup::ComputedFromLocalData(ref setup) => match
      setup.target {
          Storage::HiveTableStorage(ref storage) => match
          storage.location {
              HiveLocation::MinioLocation(ref l) => format!(
                  "{}/{}_csv",
                  l.name,
                  static_data_table.get_name(),
              ),
              _ => panic!("Only on-premise locations allowed."),
          },
          _ => panic!("Only HiveTable storage allowed."),
      },
      _ => panic!("Only StaticDataTable assets allowed."),
  }
}
,
        "features": lambda lng: lng.{
  let template = data_set.get_template_for_asset(static_data_table);
  let ptfm = match template {
      Ok(DatumTemplate::PredictionsFromTrainedFloatMeasure(x)) => x,
      _ => panic!("Template must be of type PredictionsFromTrainedFloatMeasure."),
  };
  format!("[{}]",
      ptfm.features.iter().map(|x| format!("\"{}\"", x.get_name())).collect::<Vec<String>>().join(","),
  )
}
,
    },
)
def recipe(asset_name, minio, tmp_dir, source_path, model_path, destination_path, features):
    import pandas as pd
    import pyarrow.orc as orc
    from minio import Minio
    import json
    import onnxruntime as rt
    import numpy
    
    def make_predictions(
        minio,
        tmp_dir,
        asset_name,
        source_path,
        model_path,
        destination_path,
        features,
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
        tmp_output_path = '%s/%s.onnx' % (tmp_dir, asset_name)
        client.fget_object(minio_bucket, model_path, tmp_output_path)
    
        sess = rt.InferenceSession(tmp_output_path)
        input_name = sess.get_inputs()[0].name
        label_name = sess.get_outputs()[0].name
    
        objects = client.list_objects(minio_bucket, prefix=source_path)
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
    
        input_data = pd.concat(datasets)
    
        features = json.loads(features)
        X = input_data[features].to_numpy()
        predictions = sess.run([label_name], {input_name: X.astype(numpy.float32)})[0]
        X = numpy.concatenate((X, predictions), axis=1)
        tmp_output_path = "%s/data.csv" % tmp_dir
        numpy.savetxt(tmp_output_path, X, delimiter=",")
        print("Wrote data.csv to %s" % destination_path)
        client.fput_object(minio_bucket, destination_path + "/data.csv", tmp_output_path)
    
    