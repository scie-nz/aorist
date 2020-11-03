#![allow(non_snake_case)]
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use gitea::{CreateGiteaUser, GiteaUser, Client, Error};
use crate::role::{Role, TRole};
use std::collections::HashSet;
use async_trait::async_trait;
use ranger::{CreateRangerUser, RangerClient, RangerUser, RangerError};


#[async_trait]
pub trait TGiteaEntity {
    type TCreatePayload;
    type TResultPayload;
    fn get_create_payload(&self) -> Result<Self::TCreatePayload, String>;
    async fn create(&self, client: &Client) -> Result<Self::TResultPayload, Error>;
    async fn exists(&self, client: &Client) -> Result<bool, Error>;
    async fn enforce(&mut self, client: &Client) -> Result<(), Error>;
}

#[async_trait]
pub trait TRangerEntity {
    type TCreatePayload;
    type TResultPayload;

    fn get_create_payload(&self) -> Result<Self::TCreatePayload, String>;
    async fn create(&self, client: &RangerClient) -> Result<Self::TResultPayload, RangerError>;
    async fn exists(&self, client: &RangerClient) -> Result<bool, RangerError>;
    async fn enforce(&mut self, client: &RangerClient) -> Result<(), RangerError>;
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Hash, Eq)]
pub struct User {
    firstName: String,
    lastName: String,
    email: String,
    phone: String,
    unixname: String,
    roles: Option<Vec<Role>>,
    gitea_user: Option<GiteaUser>,
    ranger_user: Option<RangerUser>,
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
    pub fn set_gitea_user(&mut self, user: GiteaUser) -> Result<(), String> {
        if let Some(_) = self.gitea_user {
            return Err("Tried to set gitea user more than once.".to_string());
        }
        self.gitea_user = Some(user);
        Ok(())
    }
    pub fn get_gitea_user(&self) -> Result<GiteaUser, String> {
        match &self.gitea_user {
            Some(x) => Ok(x.clone()),
            None => Err("Tried to get gitea_user for user but set_gitea_user was never called".to_string())
        }
    }
    pub fn set_ranger_user(&mut self, user: RangerUser) -> Result<(), String> {
        if let Some(_) = self.ranger_user {
            return Err("Tried to set ranger user more than once.".to_string());
        }
        self.ranger_user = Some(user);
        Ok(())
    }
    pub fn get_ranger_user(&self) -> Result<RangerUser, String> {
        match &self.ranger_user {
            Some(x) => Ok(x.clone()),
            None => Err("Tried to get ranger_user for user but set_ranger_user was never called".to_string())
        }
    }
}

#[async_trait]
impl TRangerEntity for User {
    type TCreatePayload = CreateRangerUser;
    type TResultPayload = RangerUser;

    fn get_create_payload(&self) -> Result<Self::TCreatePayload, String> {
        Ok(CreateRangerUser{
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
            password: format!("{}12345678", self.unixname).to_string(),
        })
    }
    async fn create(&self, client: &RangerClient) -> Result<RangerUser, RangerError> {
        println!("Creating ranger user {}", self.unixname);
        let cr = TRangerEntity::get_create_payload(self).unwrap();
        Ok(client.create_user(cr).await?)
    }
    async fn exists(&self, client: &RangerClient) -> Result<bool, RangerError> {
        println!("Checking ranger user {}", self.unixname);
        Ok(client.check_exists_username(self.unixname.clone()).await?)
    }
    async fn enforce(&mut self, client: &RangerClient) -> Result<(), RangerError> {
        while !TRangerEntity::exists(self, client).await? {
            let ranger_user = TRangerEntity::create(self, client).await?;
            self.set_ranger_user(ranger_user);
        }
        Ok(())
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
        let cr = TGiteaEntity::get_create_payload(self).unwrap();
        println!("Creating gitea user {}", self.unixname);
        Ok(client.create_user(cr).await?)
    }
    async fn exists(&self, client: &Client) -> Result<bool, Error> {
        println!("Checking gitea user {}", self.unixname);
        Ok(client.check_exists_username(self.unixname.clone()).await?)
    }
    async fn enforce(&mut self, client: &Client) -> Result<(), Error> {
        while !TGiteaEntity::exists(self, client).await? {
            let gitea_user = TGiteaEntity::create(self, client).await?;
            self.set_gitea_user(gitea_user);
        }
        Ok(())
    }
}
