use crate::hive::THiveTableCreationTagMutator;
use crate::prefect::{TObjectWithPrefectCodeGen, TPrefectHiveLocation, TPrefectLocation};
use crate::python::{TLocationWithPythonAPIClient, TObjectWithPythonCodeGen};
use enum_dispatch::enum_dispatch;
use indoc::{formatdoc, indoc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct GCSLocation {
    bucket: String,
    blob: String,
}

impl TObjectWithPrefectCodeGen for GCSLocation {
    fn get_prefect_preamble(&self, preamble: &mut HashMap<String, String>) {
        preamble.insert(
            "download_blob_to_file".to_string(),
            indoc! {"

            @task
            def download_blob_to_file(bucket_name, blob_name, file_name):
                client = storage.Client.from_service_account_json('/gcloud/social_norms.json')
                bucket = client.bucket(bucket_name)
                blob = bucket.blob(blob_name)
                blob.download_to_filename(file_name)
            "}
            .to_string(),
        );
    }
}
impl TPrefectLocation for GCSLocation {
    fn get_prefect_download_task(&self, task_name: String, file_name: String) -> String {
        format!(
            indoc! {
                "
                    {task_name} = download_blob_to_file(
                        '{bucket}',
                        '{blob}',
                        '{file_name}'
                    )
                "
            },
            task_name = task_name,
            bucket = &self.bucket,
            blob = &self.blob,
            file_name = file_name
        )
    }
}
impl TObjectWithPythonCodeGen for GCSLocation {
    fn get_python_imports(&self, preamble: &mut HashMap<String, String>) {
        let import_str = indoc!(
            "from google.cloud import storage"
        )
        .to_string();
        preamble.insert("gcs_storage_python_imports".to_string(), import_str);
        preamble.insert("prefect_import_task".to_string(), "from prefect import task, Flow".to_string());
        preamble.insert("wire_import_task".to_string(), "from alluxio import wire".to_string());
    }
}
impl TLocationWithPythonAPIClient for GCSLocation {
    fn get_python_client(&self, client_name: &String) -> String {
        formatdoc!(
            "
                {client_name} = storage.Client()
            ",
            client_name = &client_name
        )
        .to_string()
    }
    fn get_python_create_storage(&self, client_name: &String) -> String {
        formatdoc!(
            "
                {client}
                try:
                    bucket = {client_name}.get_bucket({bucket_name})
                except:
                    {client_name}.create_bucket({bucket_name})
            ",
            bucket_name = self.bucket,
            client = self.get_python_client(client_name),
            client_name = client_name
        )
        .to_string()
    }
}

#[enum_dispatch]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "spec")]
pub enum RemoteWebsiteLocation {
    GCSLocation(GCSLocation),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct HiveAlluxioLocation {
    server: String,
    port: usize,
    rest_api_port: usize,
    path: String,
}
impl TObjectWithPrefectCodeGen for HiveAlluxioLocation {
    fn get_prefect_preamble(&self, preamble: &mut HashMap<String, String>) {
        let client_name = "alluxio_client".to_string();
        preamble.insert(
            "upload_file_to_alluxio".to_string(),
            formatdoc! {"

                @task
                def upload_file_to_alluxio(local_path, remote_path, file_name):
                    {client}
                    if not {client_name}.exists(remote_path):
                        opt = alluxio.option.CreateDirectory(
                            recursive=True,
                            write_type=wire.WRITE_TYPE_CACHE_THROUGH
                        )
                        {client_name}.create_directory(remote_path, opt)
                    opt = alluxio.option.CreateFile(
                        write_type=wire.WRITE_TYPE_CACHE_THROUGH
                    )
                    with {client_name}.open(remote_path, file_name, 'w', opt) as alluxio_file:
                        with open('%s/%s' % (local_path, file_name), 'rb') as local_file:
                            alluxio_file.write(local_file)

                    ",
                client = self.get_python_client(&client_name),
                client_name = &client_name,
            }
            .to_string(),
        );
    }
}
impl TObjectWithPythonCodeGen for HiveAlluxioLocation {
    fn get_python_imports(&self, preamble: &mut HashMap<String, String>) {
        let import_str = indoc!(
            "
            import alluxio
        "
        )
        .to_string();
        preamble.insert(
            "hive_alluxio_storage_python_imports".to_string(),
            import_str,
        );
    }
}
impl TLocationWithPythonAPIClient for HiveAlluxioLocation {
    fn get_python_client(&self, client_name: &String) -> String {
        formatdoc!(
            "
                {client_name} = alluxio.Client('{server}', {port})
            ",
            client_name = client_name,
            server = self.server,
            port = self.rest_api_port
        )
        .to_string()
    }
    fn get_python_create_storage(&self, client_name: &String) -> String {
        formatdoc!(
            "
                {client}
                if not {client_name}.exists(\"{path}\"):
                    opt = alluxio.option.CreateDirectory(
                        recursive=True,
                        write_type=wire.WRITE_TYPE_CACHE_THROUGH
                    )
                    {client_name}.create_directory({path}, opt)
            ",
            client = self.get_python_client(client_name),
            client_name = client_name,
            path = self.path,
        )
        .to_string()
    }
}

impl THiveTableCreationTagMutator for HiveAlluxioLocation {
    fn populate_table_creation_tags(
        &self,
        tags: &mut HashMap<String, String>,
    ) -> Result<(), String> {
        tags.insert(
            "external_location".to_string(),
            format!(
                "alluxio://{server}:{port}/{path}",
                server = self.server,
                port = self.port,
                path = self.path
            )
            .to_string(),
        );
        Ok(())
    }
}
impl TPrefectHiveLocation for HiveAlluxioLocation {
    fn get_prefect_upload_task(
        &self,
        file_name: String,
        local_path: String,
        task_name: String,
        upstream_task_name: String,
    ) -> String {
        formatdoc!(
            "
                {task_name} = upload_file_to_alluxio(
                    '{local_path}',
                    '{remote_path}',
                    '{file_name}',
                    upstream_tasks=[{upstream_task_name}]
                )
            ",
            local_path = local_path,
            remote_path = self.path,
            task_name = task_name,
            file_name = file_name,
            upstream_task_name = upstream_task_name,
        )
        .to_string()
    }
}

#[enum_dispatch]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "spec")]
pub enum HiveLocation {
    HiveAlluxioLocation(HiveAlluxioLocation),
}
