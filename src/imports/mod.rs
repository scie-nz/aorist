#![allow(non_snake_case)]

pub mod local_import;
pub mod git_import;

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
use crate::imports::local_import::LocalFileImport;
use crate::imports::git_import::GitImport;

#[enum_dispatch(AoristImport)]
pub trait TAoristImport {
    fn get_objects(&self) -> Vec<AoristObject>;
}

#[enum_dispatch]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum AoristImport {
    LocalFileImport(LocalFileImport),
    GitImport(GitImport),
}

