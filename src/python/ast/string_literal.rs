use crate::python::ast::ArgType;
use linked_hash_map::LinkedHashMap;
use rustpython_parser::ast::{Expression, ExpressionType, Located, Location, StringGroup};
use std::collections::BTreeSet;
use std::hash::Hash;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[derive(Hash, PartialEq, Eq)]
pub struct StringLiteral {
    value: String,
    // TODO: replace with LinkedHashMap<Uuid, BTreeSet>
    object_uuids: LinkedHashMap<Uuid, BTreeSet<Option<String>>>,
    owner: Option<ArgType>,
}

impl StringLiteral {
    pub fn new(value: String) -> Self {
        Self {
            value,
            object_uuids: LinkedHashMap::new(),
            owner: None,
        }
    }
    pub fn new_wrapped(value: String) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self::new(value)))
    }
    pub fn value(&self) -> String {
        self.value.clone()
    }
    pub fn len(&self) -> usize {
        self.value.len()
    }
    pub fn register_object(&mut self, uuid: Uuid, tag: Option<String>) {
        self.object_uuids
            .entry(uuid)
            .or_insert(BTreeSet::new())
            .insert(tag);
    }
    pub fn set_owner(&mut self, obj: ArgType) {
        self.owner = Some(obj);
    }
    pub fn get_object_uuids(&self) -> &LinkedHashMap<Uuid, BTreeSet<Option<String>>> {
        &self.object_uuids
    }
    pub fn is_multiline(&self) -> bool {
        self.value.contains('\n')
    }
    pub fn get_owner(&self) -> Option<ArgType> {
        self.owner.clone()
    }
    pub fn get_ultimate_owner(&self) -> Option<ArgType> {
        if self.get_owner().is_none() {
            return None;
        }
        let mut owner = self.get_owner().unwrap();
        while let Some(x) = owner.get_owner() {
            owner = x;
        }
        Some(owner.clone())
    }
    pub fn remove_owner(&mut self) {
        self.owner = None;
    }
    pub fn expression(&self, location: Location) -> Expression {
        if let Some(ref val) = self.owner {
            return val.expression(location);
        }
        let value;
        if self.value.len() <= 60 || self.is_multiline() {
            value = StringGroup::Constant {
                value: self.value.clone(),
            };
        } else {
            let mut splits = self
                .value
                .split(",")
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .into_iter();
            let mut acc: String = splits.next().unwrap();
            let mut values: Vec<StringGroup> = Vec::new();
            for split in splits {
                if acc.len() + split.len() + 1 >= 60 {
                    values.push(StringGroup::Constant { value: acc.clone() });
                    acc = "".to_string();
                }
                acc += ",";
                acc += &split;
            }
            if acc.len() > 0 {
                values.push(StringGroup::Constant { value: acc.clone() });
            }
            value = StringGroup::Joined { values };
        }
        Located {
            location,
            node: ExpressionType::String { value },
        }
    }
    pub fn get_direct_descendants(&self) -> Vec<ArgType> {
        Vec::new()
    }
}
