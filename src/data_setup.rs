#![allow(non_snake_case)]
use crate::datasets::DataSet;
use crate::object::{AoristObject, TAoristObject};
use crate::role::{Role, TRole};
use crate::role_binding::RoleBinding;
use crate::user::User;
use crate::user_group::UserGroup;
use enum_dispatch::enum_dispatch;
use getset::{Getters, IncompleteGetters, IncompleteMutGetters, IncompleteSetters, Setters};
use git2::{Cred, RemoteCallbacks, Repository};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::path::Path;
use thiserror::Error;

fn normal_merge(
    repo: &Repository,
    local: &git2::AnnotatedCommit,
    remote: &git2::AnnotatedCommit,
) -> Result<(), git2::Error> {
    let local_tree = repo.find_commit(local.id())?.tree()?;
    let remote_tree = repo.find_commit(remote.id())?.tree()?;
    let ancestor = repo
        .find_commit(repo.merge_base(local.id(), remote.id())?)?
        .tree()?;
    let mut idx = repo.merge_trees(&ancestor, &local_tree, &remote_tree, None)?;

    if idx.has_conflicts() {
        println!("Merge conficts detected...");
        repo.checkout_index(Some(&mut idx), None)?;
        return Ok(());
    }
    let result_tree = repo.find_tree(idx.write_tree_to(repo)?)?;
    // now create the merge commit
    let msg = format!("Merge: {} into {}", remote.id(), local.id());
    let sig = repo.signature()?;
    let local_commit = repo.find_commit(local.id())?;
    let remote_commit = repo.find_commit(remote.id())?;
    // Do our merge commit and set current branch head to that commit.
    let _merge_commit = repo.commit(
        Some("HEAD"),
        &sig,
        &sig,
        &msg,
        &result_tree,
        &[&local_commit, &remote_commit],
    )?;
    // Set working tree to match head.
    repo.checkout_head(None)?;
    Ok(())
}


fn fast_forward(
    repo: &Repository,
    lb: &mut git2::Reference,
    rc: &git2::AnnotatedCommit,
) -> Result<(), git2::Error> {
    let name = match lb.name() {
        Some(s) => s.to_string(),
        None => String::from_utf8_lossy(lb.name_bytes()).to_string(),
    };
    let msg = format!("Fast-Forward: Setting {} to id: {}", name, rc.id());
    println!("{}", msg);
    lb.set_target(rc.id(), &msg)?;
    repo.set_head(&name)?;
    repo.checkout_head(Some(
        git2::build::CheckoutBuilder::default()
            // For some reason the force is required to make the working directory actually get updated
            // I suspect we should be adding some logic to handle dirty working directory states
            // but this is just an example so maybe not.
            .force(),
    ))?;
    Ok(())
}

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum GetSetError {
    #[error("Get was called, but attribute was not set: {0:#?}")]
    GetError(String),
    #[error("Set was called twice for the attribute: {0:#?}")]
    SetError(String),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PrestoConfig {
    server: String,
    httpPort: usize,
}

#[derive(Serialize, Deserialize, Clone, Getters, Setters)]
pub struct AlluxioConfig {
    #[getset(get = "pub", set = "pub")]
    server: String,
    #[getset(get = "pub", set = "pub")]
    rpcPort: usize,
    #[getset(get = "pub", set = "pub")]
    apiPort: usize,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RangerConfig {
    server: String,
    port: usize,
    user: String,
    password: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GiteaConfig {
    server: String,
    port: usize,
    token: String,
}

#[serde()]
#[derive(Serialize, Deserialize, Clone, IncompleteGetters, IncompleteSetters)]
pub struct EndpointConfig {
    #[getset(get_incomplete = "pub", set_incomplete = "pub")]
    presto: Option<PrestoConfig>,
    #[getset(get_incomplete = "pub", set_incomplete = "pub")]
    alluxio: Option<AlluxioConfig>,
    #[getset(get_incomplete = "pub", set_incomplete = "pub")]
    ranger: Option<RangerConfig>,
    #[getset(get_incomplete = "pub", set_incomplete = "pub")]
    gitea: Option<GiteaConfig>,
}

#[serde(tag = "type")]
#[derive(Serialize, Deserialize, Clone, Getters, Debug, PartialEq)]
pub struct LocalFileImport {
    #[getset(get = "pub")]
    filename: String,
}
#[serde(tag = "type")]
#[derive(Serialize, Deserialize, Clone, Getters, Debug, PartialEq)]
pub struct GitImport {
    #[getset(get = "pub")]
    keyfile: String,
    #[getset(get = "pub")]
    filename: String,
    #[getset(get = "pub")]
    sshPath: String,
    #[getset(get = "pub")]
    localPath: String,
    #[getset(get = "pub")]
    branch: String,
}
#[enum_dispatch(AoristImport)]
pub trait TAoristImport {
    fn get_objects(&self) -> Vec<AoristObject>;
}
impl TAoristImport for LocalFileImport {
    fn get_objects(&self) -> Vec<AoristObject> {
        let filename = self.filename();
        let imported_objects = read_file(&filename);
        imported_objects
    }
}
impl GitImport {
    fn get_callbacks(&self) -> RemoteCallbacks {
        // from: https://docs.rs/git2/0.13.12/git2/build/struct.RepoBuilder.html
        // Prepare callbacks.
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_url, username_from_url, _allowed_types| {
            Cred::ssh_key(
                username_from_url.unwrap(),
                None,
                std::path::Path::new(&format!("{}/.ssh/id_rsa", env::var("HOME").unwrap())),
                None,
            )
        });
        callbacks
    }
    fn get_fetch_options(&self) -> git2::FetchOptions {
        let callbacks = self.get_callbacks();
        // Prepare fetch options.
        let mut fo = git2::FetchOptions::new();
        fo.remote_callbacks(callbacks);
        fo
    }
    fn clone_repo(&self, path: &Path) {

        // Prepare builder.
        let mut builder = git2::build::RepoBuilder::new();
        builder.fetch_options(self.get_fetch_options());

        // Clone the project.
        builder
            .clone(self.filename(), path)
            .unwrap();
    }

    fn fetch_repo(&self, path: String, remote_branch: &str) {
        let repo = Repository::open(path).unwrap();
        let mut remote = repo.find_remote("origin").unwrap();
        let mut fo = self.get_fetch_options();

        // From: https://github.com/rust-lang/git2-rs/blob/master/examples/pull.rs
        // Always fetch all tags.
        // Perform a download and also update tips
        fo.download_tags(git2::AutotagOption::All);

        let refs = &["origin"];
        remote.fetch(refs, Some(&mut fo), None).unwrap();
        let fetch_head = repo.find_reference("FETCH_HEAD").unwrap();
        let fetch_commit = repo.reference_to_annotated_commit(&fetch_head).unwrap();
        let analysis = repo.merge_analysis(&[&fetch_commit]).unwrap();

		// 2. Do the appopriate merge
		if analysis.0.is_fast_forward() {
			println!("Doing a fast forward");
			// do a fast forward
			let refname = format!("refs/heads/{}", remote_branch);
			match repo.find_reference(&refname) {
				Ok(mut r) => {
					fast_forward(&repo, &mut r, &fetch_commit).unwrap();
				}
				Err(_) => {
					// The branch doesn't exist so just set the reference to the
					// commit directly. Usually this is because you are pulling
					// into an empty repository.
					repo.reference(
						&refname,
						fetch_commit.id(),
						true,
						&format!("Setting {} to {}", remote_branch, fetch_commit.id()),
					).unwrap();
					repo.set_head(&refname).unwrap();
					repo.checkout_head(Some(
						git2::build::CheckoutBuilder::default()
							.allow_conflicts(true)
							.conflict_style_merge(true)
							.force(),
					)).unwrap();
				}
			};
		} else if analysis.0.is_normal() {
			// do a normal merge
			let head_commit = repo.reference_to_annotated_commit(&repo.head().unwrap()).unwrap();
			normal_merge(&repo, &head_commit, &fetch_commit).unwrap();
		} else {
			println!("Nothing to do...");
		}
    }
}
impl TAoristImport for GitImport {
    fn get_objects(&self) -> Vec<AoristObject> {

        let path = Path::new(self.localPath());
        if !path.exists() {
            self.clone_repo(&path);
        } else {
            self.fetch_repo(self.localPath().to_string(), &self.branch());
        }

        let filename = format!("{}/{}", self.localPath(), self.filename());
        let imported_objects = read_file(&filename);
        imported_objects
    }
}

#[enum_dispatch]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum AoristImport {
    LocalFileImport(LocalFileImport),
}

#[derive(Serialize, Deserialize, Clone, Getters)]
pub struct DataSetup {
    name: String,
    users: Vec<String>,
    groups: Vec<String>,
    datasets: Vec<String>,
    role_bindings: Vec<String>,
    endpoints: EndpointConfig,
    #[getset(get = "pub")]
    imports: Option<Vec<LocalFileImport>>,
}
impl TAoristObject for DataSetup {
    fn get_name(&self) -> &String {
        &self.name
    }
}

#[derive(Serialize, Deserialize, IncompleteGetters, IncompleteSetters, IncompleteMutGetters)]
pub struct ParsedDataSetup {
    name: String,
    #[getset(
        get_incomplete = "pub with_prefix",
        set_incomplete = "pub",
        get_mut_incomplete = "pub with_prefix"
    )]
    users: Option<Vec<User>>,
    #[getset(
        get_incomplete = "pub with_prefix",
        set_incomplete = "pub",
        get_mut_incomplete = "pub with_prefix"
    )]
    groups: Option<Vec<UserGroup>>,
    #[getset(
        get_incomplete = "pub with_prefix",
        set_incomplete = "pub",
        get_mut_incomplete = "pub with_prefix"
    )]
    datasets: Option<Vec<DataSet>>,
    #[getset(
        get_incomplete = "pub with_prefix",
        set_incomplete = "pub",
        get_mut_incomplete = "pub with_prefix"
    )]
    role_bindings: Option<Vec<RoleBinding>>,
    endpoints: EndpointConfig,
}
impl ParsedDataSetup {
    pub fn get_user_unixname_map(&self) -> HashMap<String, User> {
        let users: &Vec<User> = self.get_users().unwrap();
        users
            .iter()
            .map(|x| (x.get_unixname().clone(), x.clone()))
            .collect()
    }
    pub fn get_user_permissions(&self) -> Result<HashMap<User, HashSet<String>>, String> {
        let umap = self.get_user_unixname_map();
        let mut map: HashMap<User, HashSet<String>> = HashMap::new();
        for binding in self.get_role_bindings().unwrap() {
            let name = binding.get_user_name();
            if !umap.contains_key(name) {
                return Err(format!("Cannot find user with name {}.", name));
            }
            let user = umap.get(name).unwrap().clone();
            for perm in binding.get_role().get_permissions() {
                map.entry(user.clone())
                    .or_insert_with(HashSet::new)
                    .insert(perm.clone());
            }
        }
        Ok(map)
    }
    pub fn get_pipelines(&self) -> Result<HashMap<String, String>, String> {
        let mut files: HashMap<String, String> = HashMap::new();
        for dataset in self.get_datasets().unwrap() {
            files.insert(
                dataset.get_materialize_pipeline_name(),
                dataset.get_materialize_pipeline(&self.endpoints)?,
            );
        }
        Ok(files)
    }
}

fn read_file(filename: &str) -> Vec<AoristObject> {
    let s = fs::read_to_string(filename).unwrap();
    let objects: Vec<AoristObject> = s
        .split("\n---\n")
        .filter(|x| x.len() > 0)
        .map(|x| serde_yaml::from_str(x).unwrap())
        .collect();
    objects
}

impl DataSetup {
    fn parse(self, mut objects: Vec<AoristObject>) -> ParsedDataSetup {
        println!("{}", self.imports.is_some());
        if let Some(imports) = self.imports {
            for import in imports {
                for object in import.get_objects().into_iter() {
                    objects.push(object);
                }
            }
        }

        let mut dataSetup = ParsedDataSetup {
            name: self.name,
            users: None,
            datasets: None,
            groups: None,
            role_bindings: None,
            endpoints: self.endpoints,
        };

        let user_names: HashSet<String> = self.users.iter().map(|x| x.clone()).collect();
        let dataset_names: HashSet<String> = self.datasets.iter().map(|x| x.clone()).collect();
        let group_names: HashSet<String> = self.groups.iter().map(|x| x.clone()).collect();
        let role_binding_names: HashSet<String> =
            self.role_bindings.iter().map(|x| x.clone()).collect();

        let mut users: Vec<User> = Vec::new();
        let mut groups: Vec<UserGroup> = Vec::new();
        let mut datasets: Vec<DataSet> = Vec::new();
        let mut role_bindings: Vec<RoleBinding> = Vec::new();

        for object in objects {
            match object {
                AoristObject::User(u) => {
                    if user_names.contains(u.get_name()) {
                        users.push(u)
                    }
                }
                AoristObject::DataSet(d) => {
                    if dataset_names.contains(d.get_name()) {
                        datasets.push(d)
                    }
                }
                AoristObject::UserGroup(g) => {
                    if group_names.contains(g.get_name()) {
                        groups.push(g)
                    }
                }
                AoristObject::RoleBinding(r) => {
                    if role_binding_names.contains(r.get_name()) {
                        role_bindings.push(r)
                    }
                }
                _ => {}
            }
        }
        dataSetup.set_users(users).unwrap();
        dataSetup.set_groups(groups).unwrap();
        dataSetup.set_datasets(datasets).unwrap();
        dataSetup.set_role_bindings(role_bindings).unwrap();

        let mut role_map: HashMap<String, Vec<Role>> = HashMap::new();
        for binding in dataSetup.get_role_bindings().unwrap() {
            role_map
                .entry(binding.get_user_name().clone())
                .or_insert_with(Vec::new)
                .push(binding.get_role().clone());
        }
        let mut user_map: HashMap<String, &mut User> = HashMap::new();

        for user in dataSetup.get_users_mut().unwrap().iter_mut() {
            let username = user.get_unixname();
            if role_map.contains_key(username) {
                user_map.insert(username.clone(), user);
            } else {
                user.set_roles(Vec::new()).unwrap();
            }
        }
        for (user_name, roles) in role_map.into_iter() {
            user_map
                .get_mut(&user_name)
                .unwrap()
                .set_roles(roles)
                .unwrap();
        }
        dataSetup
    }
}

pub fn get_data_setup() -> ParsedDataSetup {
    let objects = read_file("basic.yaml");
    let v: Vec<Option<&DataSetup>> = objects
        .iter()
        .map(|x| match x {
            AoristObject::DataSetup(x) => Some(x),
            _ => None,
        })
        .filter(|x| x.is_some())
        .collect();
    let dataSetup: DataSetup = v.first().unwrap().unwrap().to_owned();

    dataSetup.parse(objects)
}
