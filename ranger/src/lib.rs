#![allow(non_snake_case)]
/// The main Gitea client. You will need an API token as described [here](https://docs.gitea.io/en-us/api-usage/).
use reqwest::header;
use serde::{Deserialize, Serialize};
use std::result::Result as StdResult;
use thiserror::Error;

/// RangerError represents all of the possible errors that can happen with the Gitea
/// API. Most of these errors boil down to user error.
#[derive(Error, Debug)]
pub enum RangerError {
    #[error("error from reqwest: {0:#?}")]
    Reqwest(#[from] reqwest::Error),

    #[error("bad API token: {0:#?}")]
    BadAPIToken(#[from] reqwest::header::InvalidHeaderValue),

    #[error("error parsing/serializing json: {0:#?}")]
    Json(#[from] serde_json::Error),

    #[error("tag not found: {0}")]
    TagNotFound(String),
}

/// A handy alias for Result like `anyhow::Result`.
pub type Result<T> = StdResult<T, RangerError>;

pub struct RangerClient {
    base_url: String,
    username: String,
    password: String,
    cli: reqwest::Client,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Eq, Hash, Clone)]
pub struct CreateRangerUser {
    pub name: String,
    pub firstName: String,
    pub lastName: String,
    pub loginId: String,
    pub emailAddress: String,
    pub description: String,
    pub status: usize,
    pub isVisible: usize,
    pub groupIdList: Vec<usize>,
    pub userRoleList: Vec<String>,
    pub userSource: usize,
    pub password: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Eq, Hash, Clone)]
pub struct RangerUser {
	pub createDate: String,
	pub updateDate: String,
	pub owner: String,
	pub updatedBy: String,
    pub name: String,
    pub firstName: Option<String>,
    pub lastName: Option<String>,
    pub id: usize,
    pub emailAddress: Option<String>,
    pub description: Option<String>,
    pub status: usize,
    pub isVisible: usize,
    pub groupIdList: Option<Vec<usize>>,
    pub userRoleList: Vec<String>,
    pub userSource: usize,
}

impl RangerClient {

    pub fn new(base_url: String, username: String, password: String) -> Result<Self> {
        let mut headers = header::HeaderMap::new();
        headers.insert(header::ACCEPT, header::HeaderValue::from_str("application/json")?);
        headers.insert(header::CONTENT_TYPE, header::HeaderValue::from_str("application/json")?);

        let cli = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;
        Ok(Self {
            base_url: base_url,
            username: username,
            password: password,
            cli: cli,
        })
    }

    /// Creates a new Ranger user
    pub async fn create_user(&self, cr: CreateRangerUser) -> Result<RangerUser> {
        println!("{}", self
            .cli
            .post(&format!("{}/service/xusers/users", self.base_url))
            .json(&cr)
            .basic_auth(self.username.clone(), Some(self.password.clone()))
            .send()
            .await?.text().await?);
        Ok(self
            .cli
            .post(&format!("{}/service/xusers/users", self.base_url))
            .json(&cr)
            .basic_auth(self.username.clone(), Some(self.password.clone()))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }

    pub async fn check_exists_username(&self, username: String) -> Result<bool> {
        let res = self.get_user(username.clone()).await;
        let out = res.is_ok();
        let x = match res {
            Ok(_) =>  "is_ok".to_string(),
            Err(x) => x.to_string(),
        };
        println!("{}", x.to_string());
        Ok(out)

    }

    pub async fn get_user(&self, login: String) -> Result<RangerUser> {
        Ok(self
            .cli
            .get(&format!("{}/service/xusers/users/userName/{}", self.base_url, login))
            .basic_auth(self.username.clone(), Some(self.password.clone()))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }
}
