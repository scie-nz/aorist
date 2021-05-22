use crate::code::Preamble;
use crate::constraint_state::ConstraintState;
use crate::endpoints::EndpointConfig;
use crate::flow::{CompressibleETLTask, CompressibleTask, ETLFlow, ETLTask, StandaloneTask};
use anyhow::Result;
use aorist_ast::{SimpleIdentifier, StringLiteral, Subscript, AST};
use aorist_core::ParameterTuple;
use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;
use std::collections::{BTreeSet, HashMap};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

pub trait CodeBlockWithDefaultConstructor<T>
where
    T: ETLFlow,
    Self: CodeBlock<T>,
{
    fn new<'a>(
        members: Vec<Arc<RwLock<ConstraintState<'a>>>>,
        constraint_name: String,
        tasks_dict: Option<AST>,
        identifiers: &HashMap<Uuid, AST>,
    ) -> Result<Self>;
}

pub trait CodeBlockWithForLoopCompression<T>
where
    Self: CodeBlock<T>,
    T: ETLFlow,
    Self: Sized,
    <Self as CodeBlock<T>>::E: CompressibleETLTask<T>,
    <<Self as CodeBlock<T>>::E as ETLTask<T>>::S: CompressibleTask,
{
    fn run_task_compressions(
        compressible: LinkedHashMap<
            <<Self::E as ETLTask<T>>::S as CompressibleTask>::KeyType,
            Vec<<Self::E as ETLTask<T>>::S>,
        >,
        tasks: &mut Vec<Self::E>,
        constraint_name: String,
    );
    fn separate_compressible_tasks(
        tasks: Vec<<Self::E as ETLTask<T>>::S>,
    ) -> (
        LinkedHashMap<
            <<Self::E as ETLTask<T>>::S as CompressibleTask>::KeyType,
            Vec<<Self::E as ETLTask<T>>::S>,
        >,
        Vec<Self::E>,
    ) {
        let mut compressible = LinkedHashMap::new();
        let mut uncompressible = Vec::new();

        for task in tasks.into_iter() {
            if task.is_compressible() {
                let key = task.get_compression_key().unwrap();
                compressible.entry(key).or_insert(Vec::new()).push(task);
            } else {
                uncompressible.push(<Self::E as ETLTask<T>>::standalone_task(task));
            }
        }
        (compressible, uncompressible)
    }
}
impl<C, T: ETLFlow> CodeBlockWithDefaultConstructor<T> for C
where
    Self: CodeBlockWithForLoopCompression<T>,
    <Self as CodeBlock<T>>::E: CompressibleETLTask<T>,
    <<Self as CodeBlock<T>>::E as ETLTask<T>>::S: CompressibleTask,
{
    fn new<'a>(
        members: Vec<Arc<RwLock<ConstraintState<'a>>>>,
        constraint_name: String,
        tasks_dict: Option<AST>,
        identifiers: &HashMap<Uuid, AST>,
    ) -> Result<Self> {
        let (standalone_tasks, task_identifiers, params) = Self::create_standalone_tasks(
            members,
            constraint_name.clone(),
            tasks_dict.clone(),
            identifiers,
        )?;
        let (compressible, mut tasks) = Self::separate_compressible_tasks(standalone_tasks);
        Self::run_task_compressions(compressible, &mut tasks, constraint_name);
        Ok(Self::construct(tasks_dict, tasks, task_identifiers, params))
    }
}

pub trait CodeBlock<T>
where
    Self::P: Preamble,
    T: ETLFlow,
    Self: Sized,
    Self::E: ETLTask<T>,
{
    type P;
    type E;

    fn construct<'a>(
        tasks_dict: Option<AST>,
        tasks: Vec<Self::E>,
        task_identifiers: HashMap<Uuid, AST>,
        params: HashMap<String, Option<ParameterTuple>>,
    ) -> Self;

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
    ) -> (Vec<AST>, LinkedHashSet<Self::P>, BTreeSet<T::ImportType>);

    fn get_tasks_dict(&self) -> Option<AST>;
    fn get_identifiers(&self) -> HashMap<Uuid, AST>;
    fn get_params(&self) -> HashMap<String, Option<ParameterTuple>>;

    fn create_standalone_tasks<'a>(
        members: Vec<Arc<RwLock<ConstraintState<'a>>>>,
        constraint_name: String,
        tasks_dict: Option<AST>,
        identifiers: &HashMap<Uuid, AST>,
    ) -> Result<(
        Vec<<Self::E as ETLTask<T>>::S>,
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
            tasks.push(<Self::E as ETLTask<T>>::S::new(
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
