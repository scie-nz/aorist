#![allow(non_snake_case)]

pub mod git_import;
pub mod local_import;

use crate::imports::git_import::GitImport;
use crate::imports::local_import::LocalFileImport;
use crate::object::AoristObject;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

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
