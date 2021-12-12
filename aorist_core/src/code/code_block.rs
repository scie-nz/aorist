use crate::code::Preamble;
use crate::constraint::OuterConstraint;
use crate::constraint_state::ConstraintState;
use crate::flow::{CompressibleETLTask, CompressibleTask, ETLFlow, ETLTask, StandaloneTask};
use crate::parameter_tuple::ParameterTuple;
use crate::program::TOuterProgram;
use abi_stable::external_types::parking_lot::rw_lock::RRwLock;
use abi_stable::std_types::RArc;
use abi_stable::std_types::ROption;
use anyhow::Result;
use aorist_ast::{SimpleIdentifier, StringLiteral, Subscript, AST};
use aorist_primitives::AOption;
use aorist_primitives::{AString, AVec, AoristUniverse};
use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;
use std::collections::{BTreeSet, HashMap, HashSet};
use uuid::Uuid;

pub trait CodeBlock<'a, T, C, U, P>
where
    C: OuterConstraint<'a>,
    Self::P: Preamble,
    T: ETLFlow<U>,
    Self: Sized,
    Self::E: ETLTask<T, U>,
    U: AoristUniverse,
    P: TOuterProgram<TAncestry = C::TAncestry>,
{
    type P;
    type E;

    fn construct(
        tasks_dict: AOption<AST>,
        tasks: AVec<Self::E>,
        task_identifiers: HashMap<Uuid, AST>,
        params: HashMap<AString, AOption<ParameterTuple>>,
    ) -> Self;

    /// assigns task values (Python variables in which they will be stored)
    /// to each member of the code block.
    fn compute_task_vals(
        constraints: AVec<RArc<RRwLock<ConstraintState<'a, C, P>>>>,
        tasks_dict: &AOption<AST>,
    ) -> AVec<(AST, RArc<RRwLock<ConstraintState<'a, C, P>>>)> {
        let mut out = AVec::new();
        for rw in constraints.into_iter() {
            let read = rw.read();
            let name = read.get_task_name();
            drop(read);
            // TODO: magic number
            let task_val = match tasks_dict {
                AOption(ROption::RNone) => {
                    AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(name))
                }
                AOption(ROption::RSome(ref dict)) => {
                    /*let shorter_name =
                    name.replace(&format!("{}__", constraint_name).to_string(), "");*/

                    AST::Subscript(Subscript::new_wrapped(
                        dict.clone(),
                        AST::StringLiteral(StringLiteral::new_wrapped(name, false)),
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
        endpoints: U::TEndpoints,
    ) -> (AVec<AST>, LinkedHashSet<Self::P>, BTreeSet<T::ImportType>);

    fn get_tasks_dict(&self) -> AOption<AST>;
    fn get_identifiers(&self) -> HashMap<Uuid, AST>;
    fn get_params(&self) -> HashMap<AString, AOption<ParameterTuple>>;

    fn create_standalone_tasks(
        members: AVec<RArc<RRwLock<ConstraintState<'a, C, P>>>>,
        tasks_dict: AOption<AST>,
        identifiers: &HashMap<Uuid, AST>,
    ) -> Result<(
        AVec<<Self::E as ETLTask<T, U>>::S>,
        HashMap<Uuid, AST>,
        HashMap<AString, AOption<ParameterTuple>>,
    )> {
        let mut task_identifiers: HashMap<Uuid, AST> = HashMap::new();
        let mut params: HashMap<AString, AOption<ParameterTuple>> = HashMap::new();
        let mut tasks = AVec::new();
        let mut asts: HashSet<AST> = HashSet::new();
        for (ast, state) in Self::compute_task_vals(members, &tasks_dict) {
            if asts.contains(&ast) {
                panic!("Duplicated task val: {:?}", ast);
            }
            asts.insert(ast.clone());
            let x = state.read();
            task_identifiers.insert(x.get_constraint_uuid()?, ast.clone());
            params.insert(x.get_task_name(), x.get_params());

            let dep_uuids = x.get_dependencies()?;
            let mut dependencies = AVec::new();
            for dep in dep_uuids.iter() {
                if let Some(ident) = identifiers.get(dep) {
                    dependencies.push(ident.clone());
                } else {
                    panic!("Could not find identifier for Uuid: {}", dep);
                }
            }
            tasks.push(<Self::E as ETLTask<T, U>>::S::new(
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

pub trait CodeBlockWithDefaultConstructor<
    'a,
    T,
    C: OuterConstraint<'a>,
    U: AoristUniverse,
    P: TOuterProgram<TAncestry = C::TAncestry>,
> where
    T: ETLFlow<U>,
    Self: CodeBlock<'a, T, C, U, P>,
{
    fn new(
        members: AVec<RArc<RRwLock<ConstraintState<'a, C, P>>>>,
        constraint_name: AString,
        tasks_dict: AOption<AST>,
        identifiers: &HashMap<Uuid, AST>,
        render_dependencies: bool,
    ) -> Result<Self>;
}
pub trait CodeBlockWithForLoopCompression<
    'a,
    T,
    C: OuterConstraint<'a>,
    U: AoristUniverse,
    P: TOuterProgram<TAncestry = C::TAncestry>,
> where
    Self: CodeBlock<'a, T, C, U, P>,
    T: ETLFlow<U>,
    Self: Sized,
    <Self as CodeBlock<'a, T, C, U, P>>::E: CompressibleETLTask<T, U>,
    <<Self as CodeBlock<'a, T, C, U, P>>::E as ETLTask<T, U>>::S: CompressibleTask,
{
    fn run_task_compressions(
        compressible: LinkedHashMap<
            <<Self::E as ETLTask<T, U>>::S as CompressibleTask>::KeyType,
            AVec<<Self::E as ETLTask<T, U>>::S>,
        >,
        tasks: &mut AVec<Self::E>,
        constraint_name: AString,
        render_dependencies: bool,
    );
    fn separate_compressible_tasks(
        tasks: AVec<<Self::E as ETLTask<T, U>>::S>,
    ) -> (
        LinkedHashMap<
            <<Self::E as ETLTask<T, U>>::S as CompressibleTask>::KeyType,
            AVec<<Self::E as ETLTask<T, U>>::S>,
        >,
        AVec<Self::E>,
    ) {
        let mut compressible = LinkedHashMap::new();
        let mut uncompressible = AVec::new();

        for task in tasks.into_iter() {
            if task.is_compressible() {
                let key = task.get_compression_key().unwrap();
                compressible.entry(key).or_insert(AVec::new()).push(task);
            } else {
                uncompressible.push(<Self::E as ETLTask<T, U>>::standalone_task(task));
            }
        }
        (compressible, uncompressible)
    }
}
impl<
        'a,
        C,
        T: ETLFlow<U>,
        CType: OuterConstraint<'a>,
        U: AoristUniverse,
        P: TOuterProgram<TAncestry = CType::TAncestry>,
    > CodeBlockWithDefaultConstructor<'a, T, CType, U, P> for C
where
    Self: CodeBlockWithForLoopCompression<'a, T, CType, U, P>,
    <Self as CodeBlock<'a, T, CType, U, P>>::E: CompressibleETLTask<T, U>,
    <<Self as CodeBlock<'a, T, CType, U, P>>::E as ETLTask<T, U>>::S: CompressibleTask,
{
    fn new(
        members: AVec<RArc<RRwLock<ConstraintState<'a, CType, P>>>>,
        constraint_name: AString,
        tasks_dict: AOption<AST>,
        identifiers: &HashMap<Uuid, AST>,
        render_dependencies: bool,
    ) -> Result<Self> {
        let (standalone_tasks, task_identifiers, params) =
            Self::create_standalone_tasks(members, tasks_dict.clone(), identifiers)?;
        let (compressible, mut tasks) = Self::separate_compressible_tasks(standalone_tasks);
        Self::run_task_compressions(
            compressible,
            &mut tasks,
            constraint_name,
            render_dependencies,
        );
        Ok(Self::construct(tasks_dict, tasks, task_identifiers, params))
    }
}
