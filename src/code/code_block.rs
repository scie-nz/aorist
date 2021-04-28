use crate::code::{Import, Preamble};
use crate::constraint_state::ConstraintState;
use crate::endpoints::EndpointConfig;
use crate::flow::ETLFlow;
use crate::flow::StandaloneTask;
use crate::parameter_tuple::ParameterTuple;
use crate::python::{SimpleIdentifier, StringLiteral, Subscript, AST};
use anyhow::Result;
use linked_hash_set::LinkedHashSet;
use std::collections::{BTreeSet, HashMap};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

pub trait CodeBlock<T>
where
    Self::P: Preamble,
    Self::I: Import,
    T: ETLFlow,
    Self: Sized,
    Self::ST: StandaloneTask<T>,
{
    type P;
    type I;
    type ST;

    /// assigns task values (Python variables in which they will be stored)
    /// to each member of the code block.
    fn compute_task_vals<'a>(
        constraints: Vec<Arc<RwLock<ConstraintState<'a>>>>,
        constraint_name: &String,
        tasks_dict: &Option<AST>,
    ) -> Vec<(AST, Arc<RwLock<ConstraintState<'a>>>)> {
        let mut out = Vec::new();
        for rw in constraints.into_iter() {
            let read = rw.read().unwrap();
            let name = read.get_task_name();
            drop(read);
            // TODO: magic number
            let task_val = match tasks_dict {
                None => AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(name)),
                Some(ref dict) => {
                    let shorter_name =
                        name.replace(&format!("{}__", constraint_name).to_string(), "");

                    AST::Subscript(Subscript::new_wrapped(
                        dict.clone(),
                        AST::StringLiteral(StringLiteral::new_wrapped(shorter_name, false)),
                        false,
                    ))
                }
            };
            out.push((task_val, rw));
        }
        out
    }
    fn get_statements(
        &self,
        endpoints: &EndpointConfig,
    ) -> (Vec<AST>, LinkedHashSet<Self::P>, BTreeSet<Self::I>);

    fn get_tasks_dict(&self) -> Option<AST>;
    fn get_identifiers(&self) -> HashMap<Uuid, AST>;
    fn get_params(&self) -> HashMap<String, Option<ParameterTuple>>;
    fn new<'a>(
        members: Vec<Arc<RwLock<ConstraintState<'a>>>>,
        constraint_name: String,
        tasks_dict: Option<AST>,
        identifiers: &HashMap<Uuid, AST>,
    ) -> Result<Self>;

    fn create_standalone_tasks<'a>(
        members: Vec<Arc<RwLock<ConstraintState<'a>>>>,
        constraint_name: String,
        tasks_dict: Option<AST>,
        identifiers: &HashMap<Uuid, AST>,
    ) -> Result<(
        Vec<Self::ST>,
        HashMap<Uuid, AST>,
        HashMap<String, Option<ParameterTuple>>,
    )> {
        let mut task_identifiers: HashMap<Uuid, AST> = HashMap::new();
        let mut params: HashMap<String, Option<ParameterTuple>> = HashMap::new();
        let mut tasks = Vec::new();
        for (ast, state) in Self::compute_task_vals(members, &constraint_name, &tasks_dict) {
            let x = state.read().unwrap();
            task_identifiers.insert(x.get_constraint_uuid()?, ast.clone());
            params.insert(x.get_task_name(), x.get_params());

            let dep_uuids = x.get_dependencies()?;
            let dependencies = dep_uuids
                .iter()
                .map(|x| identifiers.get(x).unwrap().clone())
                .collect();
            tasks.push(Self::ST::new(
                x.get_task_name(),
                ast.clone(),
                x.get_call(),
                x.get_params(),
                dependencies,
                x.get_preamble(),
                x.get_dialect(),
            ));
        }
        Ok((tasks, task_identifiers, params))
    }
}
