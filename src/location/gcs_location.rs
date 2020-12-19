use crate::concept::AoristConcept;
use crate::constraint::Constraint;
use crate::endpoints::EndpointConfig;
use crate::prefect::{TObjectWithPrefectCodeGen, TPrefectLocation};
use crate::python::{TLocationWithPythonAPIClient, TObjectWithPythonCodeGen};
use aorist_concept::Constrainable;
use derivative::Derivative;
use indoc::{formatdoc, indoc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;

#[derive(Derivative, Serialize, Deserialize, Clone, Constrainable)]
#[derivative(PartialEq, Debug)]
pub struct GCSLocation {
    // TODO: replace these with Getters and Setters
    pub bucket: String,
    pub blob: String,
    uuid: Option<Uuid>,
    #[serde(skip)]
    #[derivative(PartialEq = "ignore", Debug = "ignore")]
    pub constraints: Vec<Rc<Constraint>>,
}

impl TObjectWithPrefectCodeGen for GCSLocation {
    fn get_prefect_preamble(
        &self,
        preamble: &mut HashMap<String, String>,
        _endpoints: &EndpointConfig,
    ) {
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
        let import_str = indoc!("from google.cloud import storage").to_string();
        preamble.insert("gcs_storage_python_imports".to_string(), import_str);
        preamble.insert(
            "prefect_import_task".to_string(),
            "from prefect import task, Flow".to_string(),
        );
        preamble.insert(
            "wire_import_task".to_string(),
            "from alluxio import wire".to_string(),
        );
    }
}
impl TLocationWithPythonAPIClient for GCSLocation {
    fn get_python_client(
        &self,
        client_name: &String,
        // TODO: add GCS credentials to EndpointConfig
        _endpoints: &EndpointConfig,
    ) -> String {
        formatdoc!(
            "
                {client_name} = storage.Client()
            ",
            client_name = &client_name
        )
        .to_string()
    }
    fn get_python_create_storage(
        &self,
        client_name: &String,
        endpoints: &EndpointConfig,
    ) -> String {
        formatdoc!(
            "
                {client}
                try:
                    bucket = {client_name}.get_bucket({bucket_name})
                except:
                    {client_name}.create_bucket({bucket_name})
            ",
            bucket_name = self.bucket,
            client = self.get_python_client(client_name, endpoints),
            client_name = client_name
        )
        .to_string()
    }
}
