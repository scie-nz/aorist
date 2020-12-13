use serde::{Deserialize, Serialize};
use crate::object::TAoristObject;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Hash, Eq)]
pub struct Constraint {
    name: String,
}
impl TAoristObject for Constraint {
    fn get_name(&self) -> &String {
        &self.name
    }
}

