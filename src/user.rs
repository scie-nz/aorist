#![allow(non_snake_case)]
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::ranger::RangerEntity;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Hash, Eq)]
pub struct User {
    firstName: String,
    lastName: String,
    email: String,
    phone: String,
    unixname: String,
}
impl User {
    pub fn to_yaml(&self) -> String {
        serde_yaml::to_string(self).unwrap()
    }
    pub fn get_unixname(&self) -> &String {
        &self.unixname
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UserRangerPayload {
    name: String,
    firstName: String,
    lastName: String,
    loginId: String,
    emailAddress: String,
    description: String,
    status: usize,
    isVisible: usize,
    groupIdList: Vec<usize>,
    userRoleList: Vec<String>,
    userSource: usize,
}

impl RangerEntity for User {
    type TRangerCreatePayload = UserRangerPayload;

    fn get_ranger_create_endpoint(&self) -> String {
        "service/xusers/secure/users".to_string()
    }
    fn get_ranger_create_headers(&self) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        headers.insert("Accept".to_string(), "application/json".to_string());
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers
    }
    fn get_ranger_create_payload(&self) -> UserRangerPayload {
        UserRangerPayload{
            name: self.unixname.clone(),
            firstName: self.firstName.clone(),
            lastName: self.lastName.clone(),
            loginId: self.unixname.clone(),
            emailAddress: self.email.clone(),
            description: "External user account".to_string(),
            status: 1,
            isVisible: 1,
            groupIdList: Vec::new(),
            userRoleList: vec!["ROLE_USER".to_string()],
            userSource: 0,
        }
    }
}

