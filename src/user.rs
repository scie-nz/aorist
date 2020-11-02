#![allow(non_snake_case)]
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::ranger::RangerEntity;
use gitea::{CreateGiteaUser, GiteaUser, Client, Error};
use crate::role::{Role, TRole};
use std::collections::HashSet;
use async_trait::async_trait;

#[async_trait]
pub trait TGiteaEntity {
    type TCreatePayload;
    type TResultPayload;
    fn get_create_payload(&self) -> Result<Self::TCreatePayload, String>;
    async fn create(&self, client: &Client) -> Result<Self::TResultPayload, Error>;
    async fn exists(&self, client: &Client) -> Result<bool, Error>;
    async fn enforce(&self, client: &Client) -> Result<(), Error> {
        while !self.exists(client).await? {
            self.enforce(client).await?
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Hash, Eq)]
pub struct User {
    firstName: String,
    lastName: String,
    email: String,
    phone: String,
    unixname: String,
    roles: Option<Vec<Role>>,
}
impl User {
    pub fn to_yaml(&self) -> String {
        serde_yaml::to_string(self).unwrap()
    }
    pub fn get_unixname(&self) -> &String {
        &self.unixname
    }
    pub fn set_roles(&mut self, roles: Vec<Role>) -> Result<(), String> {
        if let Some(_) = self.roles {
            return Err("Tried to set roles more than once.".to_string());
        }
        self.roles = Some(roles);
        Ok(())
    }
    pub fn get_roles(&self) -> Result<Vec<Role>, String> {
        match &self.roles {
            Some(x) => Ok(x.clone()),
            None => Err("Tried to get roles for user but set_roles was never called".to_string())
        }
    }
    pub fn get_permissions(&self) -> Result<HashSet<String>, String> {
        let mut perms: HashSet<String> = HashSet::new();
        for role in self.get_roles()? {
            for perm in role.get_permissions() {
                perms.insert(perm);
            }
        }
        Ok(perms)
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

#[async_trait]
impl TGiteaEntity for User {
    type TCreatePayload = CreateGiteaUser;
    type TResultPayload = GiteaUser;

    fn get_create_payload(&self) -> Result<CreateGiteaUser, String> {
        Ok(CreateGiteaUser {
            email: self.email.clone(),
            full_name: format!("{} {}", self.firstName, self.lastName).to_string(),
            login_name: self.unixname.clone(),
            username: self.unixname.clone(),
            password: format!("{}123", self.unixname).to_string(),
            send_notify: false,
            source_id: 0,
            must_change_password: false,
        })
    }
    async fn create(&self, client: &Client) -> Result<GiteaUser, Error> {
        let cr = self.get_create_payload().unwrap();
        println!("Creating gitea user {}", self.unixname);
        Ok(client.create_user(cr).await?)
    }
    async fn exists(&self, client: &Client) -> Result<bool, Error> {
        println!("Checking gitea user {}", self.unixname);
        Ok(client.check_exists_username(self.unixname.clone()).await?)
    }
}
