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

pub fn read_file(filename: &str) -> Vec<AoristObject> {
    let s = fs::read_to_string(filename).unwrap();
    let objects: Vec<AoristObject> = s
        .split("\n---\n")
        .filter(|x| x.len() > 0)
        .map(|x| serde_yaml::from_str(x).unwrap())
        .collect();
    objects
}

