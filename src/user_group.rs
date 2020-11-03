#![allow(non_snake_case)]
//use crate::ranger::RangerEntity;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UserGroup {
    name: String,
    members: Vec<String>,
    labels: HashMap<String, String>,
    description: Option<String>,
}
impl UserGroup {
    pub fn to_yaml(&self) -> String {
        serde_yaml::to_string(self).unwrap()
    }
    pub fn get_labels(&self) -> &HashMap<String, String> {
        &self.labels
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UserGroupRangerPayload {
    name: String,
    description: String,
}
/*
impl RangerEntity for UserGroup {
    type TRangerCreatePayload = UserGroupRangerPayload;

    fn get_ranger_create_endpoint(&self) -> String {
        "service/xusers/secure/groups".to_string()
    }
    fn get_ranger_create_headers(&self) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        headers.insert("Accept".to_string(), "application/json".to_string());
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("X-XSRF-HEADER".to_string(), "".to_string());
        headers.insert("X-Requested-With".to_string(), "XMLHttpRequest".to_string());
        headers
    }
    fn get_ranger_create_payload(&self) -> UserGroupRangerPayload {
        UserGroupRangerPayload{
            name: self.name.clone(),
            description: match &self.description {
                Some(x) => x.to_string(),
                None => "".to_string()
            },
        }
    }
}*/
