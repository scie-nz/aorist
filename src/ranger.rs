#![allow(non_snake_case)]
use serde::Serialize;
use std::collections::HashMap;
pub trait RangerEntity
where <Self as RangerEntity>::TRangerCreatePayload: Serialize {
    type TRangerCreatePayload;
    fn get_ranger_create_endpoint(&self) -> String;
    fn get_ranger_create_headers(&self) -> HashMap<String, String>;
    fn get_ranger_create_payload(&self) -> Self::TRangerCreatePayload;

    fn get_range_create_curl(
        &self,
        username: String,
        password: String,
        hostname: String,
        port: usize,
    ) -> String {
        format!(
            "curl -v -u {userblock} {endpoint} {headers} --data-binary '{payload}'",
            userblock=format!("{user}:{password}", user=username, password=password),
            endpoint=format!("http://{hostname}:{port}/{endpoint}",
                             hostname=hostname,
                             port=port,
                             endpoint=self.get_ranger_create_endpoint()),
            headers=self
                .get_ranger_create_headers()
                .iter()
                .map(|(k, v)| format!("-H '{}: {}'", k, v))
                .collect::<Vec<String>>()
                .join(" "),
            payload=serde_json::to_string(&self.get_ranger_create_payload()).unwrap(),
        )
    }
}

