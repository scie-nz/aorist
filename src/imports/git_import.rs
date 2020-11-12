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
use crate::imports::TAoristImport;
use crate::utils::read_file;

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

