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

#[serde(tag = "type")]
#[derive(Serialize, Deserialize, Clone, Getters, Debug, PartialEq)]
pub struct LocalFileImport {
    #[getset(get = "pub")]
    filename: String,
}

impl TAoristImport for LocalFileImport {
    fn get_objects(&self) -> Vec<AoristObject> {
        let filename = self.filename();
        let imported_objects = read_file(&filename);
        imported_objects
    }
}
