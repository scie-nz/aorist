#![allow(non_snake_case)]
use crate::constraint::Constraint;
use crate::data_setup::parsed_data_setup::ParsedDataSetup;
use crate::dataset::DataSet;
use crate::endpoints::EndpointConfig;
use crate::imports::local_import::LocalFileImport;
use crate::imports::TAoristImport;
use crate::object::{AoristObject, TAoristObject};
use crate::role::Role;
use crate::role_binding::RoleBinding;
use crate::user::User;
use crate::user_group::UserGroup;
use getset::Getters;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};

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
    constraints: Vec<String>,
}
impl TAoristObject for DataSetup {
    fn get_name(&self) -> &String {
        &self.name
    }
}

impl DataSetup {
    pub fn parse(self, mut objects: Vec<AoristObject>) -> ParsedDataSetup {
        println!("{}", self.imports.is_some());
        if let Some(imports) = self.imports {
            for import in imports {
                for object in import.get_objects().into_iter() {
                    objects.push(object);
                }
            }
        }

        let mut dataSetup = ParsedDataSetup::new(self.name, self.endpoints);

        let user_names: HashSet<String> = self.users.iter().map(|x| x.clone()).collect();
        let dataset_names: HashSet<String> = self.datasets.iter().map(|x| x.clone()).collect();
        let group_names: HashSet<String> = self.groups.iter().map(|x| x.clone()).collect();
        let role_binding_names: HashSet<String> =
            self.role_bindings.iter().map(|x| x.clone()).collect();
        let constraint_names: HashSet<String> = self.constraints.iter().cloned().collect();

        let mut users: Vec<User> = Vec::new();
        let mut groups: Vec<UserGroup> = Vec::new();
        let mut datasets: Vec<DataSet> = Vec::new();
        let mut role_bindings: Vec<RoleBinding> = Vec::new();
        let mut constraints: Vec<Arc<RwLock<Constraint>>> = Vec::new();

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
                AoristObject::Constraint(c) => {
                    if constraint_names.contains(c.get_name()) {
                        constraints.push(Arc::new(RwLock::new(c)))
                    }
                }
                _ => {}
            }
        }
        dataSetup.set_users(users).unwrap();
        dataSetup.set_groups(groups).unwrap();
        dataSetup.set_datasets(datasets).unwrap();
        dataSetup.set_role_bindings(role_bindings).unwrap();
        //dataSetup.set_constraints(constraints);

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
