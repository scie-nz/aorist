/// The main Gitea client. You will need an API token as described [here](https://docs.gitea.io/en-us/api-usage/).
use reqwest::header;
use serde::{Deserialize, Serialize};
use std::result::Result as StdResult;
use thiserror::Error;

/// Error represents all of the possible errors that can happen with the Gitea
/// API. Most of these errors boil down to user error.
#[derive(Error, Debug)]
pub enum Error {
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
pub type Result<T> = StdResult<T, Error>;

/// A repository release.
/// https://try.gitea.io/api/swagger#model-Release
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Release {
    pub id: i64,
    pub tag_name: String,
    pub target_commitish: String,
    pub name: String,
    pub body: String,
    pub url: String,
    pub tarball_url: String,
    pub zipball_url: String,
    pub draft: bool,
    pub prerelease: bool,
    pub created_at: String,
    pub published_at: String,
    pub author: GiteaUser,
    pub assets: Vec<Attachment>,
}

/// The inputs to create a repository release.
/// https://try.gitea.io/api/swagger#model-CreateReleaseOption
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateRelease {
    pub body: String,
    pub draft: bool,
    pub name: String,
    pub prerelease: bool,
    pub tag_name: String,
    pub target_commitish: String,
}

/// An attachment to a release, such as a pre-built package.
/// https://try.gitea.io/api/swagger#model-Attachment
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attachment {
    pub id: i64,
    pub name: String,
    pub size: i64,
    pub download_count: i64,
    pub created_at: String,
    pub uuid: String,
    pub browser_download_url: String,
}

/// Inputs to create a gitea repo.
/// https://try.gitea.io/api/swagger#model-CreateRepoOption
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateRepo {
    pub auto_init: bool,
    pub description: String,
    pub gitignores: String,
    pub issue_labels: String,
    pub license: String,
    pub name: String,
    pub private: bool,
    pub readme: String,
}

/// A git repository.
/// https://try.gitea.io/api/swagger#model-Repository
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Repo {
    pub allow_merge_commits: bool,
    pub allow_rebase: bool,
    pub allow_rebase_explicit: bool,
    pub allow_squash_merge: bool,
    pub archived: bool,
    pub avatar_url: String,
    pub clone_url: String,
    pub created_at: String,
    pub default_branch: String,
    pub description: String,
    pub empty: bool,
    pub fork: bool,
    pub forks_count: i64,
    pub full_name: String,
    pub has_issues: bool,
    pub has_pull_requests: bool,
    pub has_wiki: bool,
    pub html_url: String,
    pub id: i64,
    pub ignore_whitespace_conflicts: bool,
    pub mirror: bool,
    pub name: String,
    pub open_issues_count: i64,
    pub open_pr_counter: i64,
    pub original_url: String,
    pub owner: GiteaUser,
    pub permissions: Permissions,
    pub private: bool,
    pub release_counter: i64,
    pub size: i64,
    pub ssh_url: String,
    pub stars_count: i64,
    pub template: bool,
    pub updated_at: String,
    pub watchers_count: i64,
    pub website: String,
}

/// A user.
/// https://try.gitea.io/api/swagger#model-GiteaUser
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GiteaUser {
    pub avatar_url: String,
    pub created: String,
    pub email: String,
    pub full_name: String,
    pub id: i64,
    pub is_admin: bool,
    pub language: String,
    pub last_login: String,
    pub login: String,
}

/// Inputs to create a gitea user.
/// https://try.gitea.io/api/swagger#/admin/adminCreateGiteaUser
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateGiteaUser {
 	pub email: String,
    pub full_name: String,
	pub login_name: String,
	pub must_change_password: bool,
	pub password: String,
	pub send_notify: bool,
	pub source_id: i64,
	pub username: String,
}

/// The permission set that a given user has on a Repo.
/// https://try.gitea.io/api/swagger#model-Permission
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Permissions {
    pub admin: bool,
    pub pull: bool,
    pub push: bool,
}

/// The version of Gitea.
/// https://try.gitea.io/api/swagger#model-ServerVersion
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Version {
    pub version: String,
}

/// The gitea client that all gitea calls will go through. This wraps
/// [reqwest::Client](https://docs.rs/reqwest/0.10.6/reqwest/struct.Client.html)
/// and operates asyncronously.
pub struct Client {
    cli: reqwest::Client,
    base_url: String,
}

impl Client {
    /// Create a new API client with the given base URL, token and user agent.
    /// If you need inspiration for a user agent, try this:
    ///
    /// ```rust
    /// const APP_USER_AGENT: &'static str =
    ///    concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
    /// gitea::Client::new("https://tulpa.dev".into(), "ayylmao".into(), APP_USER_AGENT).unwrap();
    /// ```
    pub fn new<T>(base_url: String, token: String, user_agent: T) -> Result<Self>
    where
        T: Into<String>,
    {
        let mut headers = header::HeaderMap::new();
        let auth = format!("token {}", token);
        let auth = auth.as_str();
        headers.insert(header::AUTHORIZATION, header::HeaderValue::from_str(auth)?);
        let cli = reqwest::Client::builder()
            .user_agent(user_agent.into())
            .default_headers(headers)
            .build()?;
        Ok(Self {
            cli: cli,
            base_url: base_url,
        })
    }

    /// Gets the current version of gitea.
    ///
    /// ```rust
    /// use gitea::Result;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let cli = gitea::Client::new("https://tulpa.dev".into(), "ayylmao".into(), "test/test")?;
    ///     println!("{:?}", cli.version().await?);
    ///     Ok(())
    /// }
    /// ```
    pub async fn version(&self) -> Result<Version> {
        Ok(self
            .cli
            .get(&format!("{}/api/v1/version", self.base_url))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }

    /// Gets a release of a repo by its tag name.
    ///
    /// ```rust
    /// use gitea::Result;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let cli = gitea::Client::new("https://tulpa.dev".into(), "ayylmao".into(), "test/test")?;
    ///     let release = cli.get_release_by_tag("cadey".into(), "gitea-release".into(), "0.3.2".into()).await;
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_release_by_tag(
        &self,
        owner: String,
        repo: String,
        tag: String,
    ) -> Result<Release> {
        let releases: Vec<Release> = self.get_releases(owner, repo).await?;
        let mut release: Option<Release> = None;

        for rls in releases {
            if *tag == rls.tag_name {
                release = Some(rls);
            }
        }

        match release {
            None => Err(Error::TagNotFound(tag)),
            Some(release) => Ok(release),
        }
    }

    /// Creates a new gitea repo for the currently authenticated user with given details.
    pub async fn create_user_repo(&self, cr: CreateRepo) -> Result<Repo> {
        Ok(self
            .cli
            .post(&format!("{}/api/v1/user/repos", self.base_url))
            .json(&cr)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }

    /// Creates a new gitea repo for a given organization with given details.
    pub async fn create_org_repo(&self, org: String, cr: CreateRepo) -> Result<Repo> {
        Ok(self
            .cli
            .post(&format!("{}/api/v1/org/{}/repos", self.base_url, org))
            .json(&cr)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }

    /// Deletes a gitea repo by owner and name.
    pub async fn delete_repo(&self, owner: String, repo: String) -> Result<()> {
        self.cli
            .delete(&format!(
                "{}/api/v1/repos/{}/{}",
                self.base_url, owner, repo
            ))
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Gets a gitea repo by name.
    ///
    /// ```rust
    /// use gitea::Result;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let cli = gitea::Client::new("https://tulpa.dev".into(), "ayylmao".into(), "test/test")?;
    ///     let repo = cli.get_repo("cadey".into(), "gitea-release".into()).await;
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_repo(&self, owner: String, repo: String) -> Result<Repo> {
        Ok(self
            .cli
            .get(&format!(
                "{}/api/v1/repos/{}/{}",
                self.base_url, owner, repo
            ))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }

    /// Gets all of the releases for a given gitea repo.
    ///
    /// ```rust
    /// use gitea::Result;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let cli = gitea::Client::new("https://tulpa.dev".into(), "ayylmao".into(), "test/test")?;
    ///     let repo = cli.get_releases("cadey".into(), "gitea-release".into()).await;
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_releases(&self, owner: String, repo: String) -> Result<Vec<Release>> {
        Ok(self
            .cli
            .get(&format!(
                "{}/api/v1/repos/{}/{}/releases",
                self.base_url, owner, repo
            ))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }

    /// Creates a new gitea release.
    ///
    /// ```rust
    /// use gitea::{CreateRelease, Result};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let cli = gitea::Client::new("https://tulpa.dev".into(), "ayylmao".into(), "test/test")?;
    ///     let repo = cli.create_release(
    ///         "cadey".into(),
    ///         "gitea-release".into(),
    ///         CreateRelease{
    ///             body: "This is a cool release".into(),
    ///             draft: false,
    ///             name: "test".into(),
    ///             prerelease: false,
    ///             tag_name: "v4.2.0".into(),
    ///             target_commitish: "HEAD".into(),
    ///         },
    ///     ).await;
    ///     Ok(())
    /// }
    /// ```
    pub async fn create_release(
        &self,
        owner: String,
        repo: String,
        cr: CreateRelease,
    ) -> Result<Release> {
        Ok(self
            .cli
            .post(&format!(
                "{}/api/v1/repos/{}/{}/releases",
                self.base_url, owner, repo
            ))
            .json(&cr)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }

    /// Deletes a given release by tag name.
    ///
    /// ```rust
    /// use gitea::Result;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let cli = gitea::Client::new("https://tulpa.dev".into(), "ayylmao".into(), "test/test")?;
    ///     let _ = cli.delete_release("cadey".into(), "gitea-release".into(), "4.2.0".into()).await;
    ///     Ok(())
    /// }
    /// ```
    pub async fn delete_release(&self, owner: String, repo: String, tag: String) -> Result<()> {
        let release = self
            .get_release_by_tag(owner.clone(), repo.clone(), tag)
            .await?;

        self.cli
            .delete(&format!(
                "{}/api/v1/repos/{}/{}/releases/{}",
                self.base_url, owner, repo, release.id
            ))
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Returns information about the currently authenticated user.
    pub async fn whoami(&self) -> Result<GiteaUser> {
        Ok(self
            .cli
            .get(&format!("{}/api/v1/user", self.base_url))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }

    /// Creates a new user
    pub async fn create_user(&self, cr: CreateGiteaUser) -> Result<GiteaUser> {
        Ok(self
            .cli
            .post(&format!("{}/admin/users", self.base_url))
            .json(&cr)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }

    pub async fn list_all_users(&self) -> Result<Vec<GiteaUser>> {
        Ok(self
            .cli
            .get(&format!("{}/admin/users", self.base_url))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }

    pub async fn check_exists_username(&self, username: String) -> Result<bool> {
        let users: Vec<GiteaUser> = self.list_all_users().await?;
        for user in users {
            if user.login == username {
                return Ok(true);
            }
        }
        Ok(false)
    }
}
