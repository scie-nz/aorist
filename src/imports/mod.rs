#![allow(non_snake_case)]

pub mod local_import;
pub mod git_import;

use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use crate::imports::local_import::LocalFileImport;
use crate::imports::git_import::GitImport;
use crate::object::AoristObject;

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

