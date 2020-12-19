use crate::concept::AoristConcept;
use crate::constraint::Constraint;
use crate::endpoints::EndpointConfig;
use crate::hive::THiveTableCreationTagMutator;
use crate::prefect::{TObjectWithPrefectCodeGen, TPrefectHiveLocation};
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
pub struct AlluxioLocation {
    path: String,
    uuid: Option<Uuid>,
    #[serde(skip)]
    #[derivative(PartialEq = "ignore", Debug = "ignore")]
    pub constraints: Vec<Rc<Constraint>>,
}
impl TObjectWithPrefectCodeGen for AlluxioLocation {
    fn get_prefect_preamble(
        &self,
        preamble: &mut HashMap<String, String>,
        endpoints: &EndpointConfig,
    ) {
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
                client = self.get_python_client(&client_name, endpoints),
                client_name = &client_name,
            }
            .to_string(),
        );
    }
}
impl TObjectWithPythonCodeGen for AlluxioLocation {
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
impl TLocationWithPythonAPIClient for AlluxioLocation {
    fn get_python_client(&self, client_name: &String, endpoints: &EndpointConfig) -> String {
        formatdoc!(
            "
                {client_name} = alluxio.Client('{server}', {port})
            ",
            client_name = client_name,
            server = endpoints.alluxio().unwrap().server(),
            port = endpoints.alluxio().unwrap().apiPort(),
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
                if not {client_name}.exists(\"{path}\"):
                    opt = alluxio.option.CreateDirectory(
                        recursive=True,
                        write_type=wire.WRITE_TYPE_CACHE_THROUGH
                    )
                    {client_name}.create_directory({path}, opt)
            ",
            client = self.get_python_client(client_name, endpoints),
            client_name = client_name,
            path = self.path,
        )
        .to_string()
    }
}

impl THiveTableCreationTagMutator for AlluxioLocation {
    fn populate_table_creation_tags(
        &self,
        tags: &mut HashMap<String, String>,
        endpoints: &EndpointConfig,
    ) -> Result<(), String> {
        tags.insert(
            "external_location".to_string(),
            format!(
                "alluxio://{server}:{port}/{path}",
                server = endpoints.alluxio().unwrap().server(),
                port = endpoints.alluxio().unwrap().rpcPort(),
                path = self.path
            )
            .to_string(),
        );
        Ok(())
    }
}
impl TPrefectHiveLocation for AlluxioLocation {
    fn get_prefect_upload_task(
        &self,
        file_name: String,
        local_path: String,
        task_name: String,
        upstream_task_name: String,
        _endpoints: &EndpointConfig,
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
