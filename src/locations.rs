use indoc::indoc;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct GCSLocation {
    bucket: String,
    blob: String,
}

impl GCSLocation {
    pub fn get_prefect_preamble(&self) -> HashMap<String, String> {
        let mut preamble = HashMap::new();
        preamble.insert(
            "download_blob_to_file".to_string(),
            indoc! {"
            @task
            def download_blob_to_file(bucket_name, blob_name, file_name):
                client = storage.Client()
                bucket = client.bucket(bucket_name)
                blob = bucket.blob(blob_name)
                blob.download_to_filename(file_name)
            "}.to_string()
        );
        preamble
    }
    pub fn get_prefect_download_task(&self, task_name: String, file_name: String) -> String {
        format!(
            indoc! {
                "
                    {task_name} = download_blob_to_file(
			            '{bucket}',
                        '{blob},
                        '{file_name}'
                    )
                "
            },
            task_name=task_name,
            bucket=&self.bucket,
            blob=&self.blob,
            file_name=file_name
        )
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content="spec")]
pub enum RemoteWebsiteLocation {
    GCSLocation(GCSLocation),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct HiveAlluxioLocation {
    server: String,
    port: usize,
    path: String,
}
impl HiveAlluxioLocation {
    pub fn populate_table_creation_tags(&self, tags: &mut HashMap<String, String>) -> Result<(), String> {
        tags.insert(
            "external_location".to_string(), 
            format!("alluxio://{server}:{port}/{path}",
                    server=self.server,
                    port=self.port,
                    path=self.path).to_string()
        );
        Ok(())
    }
}
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content="spec")]
pub enum HiveLocation {
    HiveAlluxioLocation(HiveAlluxioLocation),
}

impl HiveLocation {
    pub fn populate_table_creation_tags(&self, tags: &mut HashMap<String, String>) -> Result<(), String> {
        match self {
            HiveLocation::HiveAlluxioLocation(x) => x.populate_table_creation_tags(tags)
        }
    }
}
