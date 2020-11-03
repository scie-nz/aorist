#![allow(non_snake_case)]
use crate::error::AoristError;
use crate::object::TAoristObject;
use crate::ranger::TRangerEntity;
use crate::role::{Role, TRole};
use async_trait::async_trait;
use gitea::{Client, CreateGiteaUser, GiteaUser};
use ranger::{CreateRangerUser, RangerClient, RangerUser};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[async_trait]
pub trait TGiteaEntity {
    type TCreatePayload;
    type TResultPayload;
    fn get_create_payload(&self) -> Result<Self::TCreatePayload, String>;
    async fn create(&self, client: &Client) -> Result<Self::TResultPayload, AoristError>;
    async fn exists(&self, client: &Client) -> Result<bool, AoristError>;
    async fn enforce(&mut self, client: &Client) -> Result<(), AoristError>;
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
    pub fn set_roles(&mut self, roles: Vec<Role>) -> Result<(), AoristError> {
        if let Some(_) = self.roles {
            return Err(AoristError::OtherError(
                "Tried to set roles more than once.".to_string(),
            ));
        }
        self.roles = Some(roles);
        Ok(())
    }
    pub fn get_roles(&self) -> Result<Vec<Role>, AoristError> {
        match &self.roles {
            Some(x) => Ok(x.clone()),
            None => Err(AoristError::OtherError(
                "Tried to get roles for user but set_roles was never called".to_string(),
            )),
        }
    }
    pub fn get_permissions(&self) -> Result<HashSet<String>, AoristError> {
        let mut perms: HashSet<String> = HashSet::new();
        for role in self.get_roles()? {
            for perm in role.get_permissions() {
                perms.insert(perm);
            }
        }
        Ok(perms)
    }
    pub fn set_gitea_user(&mut self, user: GiteaUser) -> Result<(), AoristError> {
        if let Some(_) = self.gitea_user {
            return Err(AoristError::OtherError(
                "Tried to set gitea user more than once.".to_string(),
            ));
        }
        self.gitea_user = Some(user);
        Ok(())
    }
    pub fn get_gitea_user(&self) -> Result<GiteaUser, AoristError> {
        match &self.gitea_user {
            Some(x) => Ok(x.clone()),
            None => Err(AoristError::OtherError(
                "Tried to get gitea_user for user but set_gitea_user was never called".to_string(),
            )),
        }
    }
    pub fn set_ranger_user(&mut self, user: RangerUser) -> Result<(), AoristError> {
        if let Some(_) = self.ranger_user {
            return Err(AoristError::OtherError(
                "Tried to set ranger user more than once.".to_string(),
            ));
        }
        self.ranger_user = Some(user);
        Ok(())
    }
    pub fn get_ranger_user(&self) -> Result<RangerUser, AoristError> {
        match &self.ranger_user {
            Some(x) => Ok(x.clone()),
            None => Err(AoristError::OtherError(
                "Tried to get ranger_user for user but set_ranger_user was never called"
                    .to_string(),
            )),
        }
    }
}
impl TAoristObject for User {
    fn get_name(&self) -> &String {
        &self.unixname
    }
}

#[async_trait]
impl TRangerEntity for User {
    type TCreatePayload = CreateRangerUser;
    type TResultPayload = RangerUser;

    fn get_create_payload(&self) -> Result<Self::TCreatePayload, String> {
        Ok(CreateRangerUser {
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
    async fn create(&self, client: &RangerClient) -> Result<RangerUser, AoristError> {
        println!("Creating ranger user {}", self.unixname);
        let cr = TRangerEntity::get_create_payload(self).unwrap();
        Ok(client.create_user(cr).await?)
    }
    async fn exists(&self, client: &RangerClient) -> Result<bool, AoristError> {
        println!("Checking ranger user {}", self.unixname);
        Ok(client.check_exists_username(self.unixname.clone()).await?)
    }
    async fn enforce(&mut self, client: &RangerClient) -> Result<(), AoristError> {
        while !TRangerEntity::exists(self, client).await? {
            let ranger_user = TRangerEntity::create(self, client).await?;
            self.set_ranger_user(ranger_user)?;
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
    async fn create(&self, client: &Client) -> Result<GiteaUser, AoristError> {
        let cr = TGiteaEntity::get_create_payload(self).unwrap();
        println!("Creating gitea user {}", self.unixname);
        Ok(client.create_user(cr).await?)
    }
    async fn exists(&self, client: &Client) -> Result<bool, AoristError> {
        println!("Checking gitea user {}", self.unixname);
        Ok(client.check_exists_username(self.unixname.clone()).await?)
    }
    async fn enforce(&mut self, client: &Client) -> Result<(), AoristError> {
        while !TGiteaEntity::exists(self, client).await? {
            let gitea_user = TGiteaEntity::create(self, client).await?;
            self.set_gitea_user(gitea_user)?;
        }
        Ok(())
    }
}
