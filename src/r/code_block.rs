#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::code::{CodeBlock, CodeBlockWithForLoopCompression};
use crate::endpoints::EndpointConfig;
use crate::flow::{CompressibleTask, ETLFlow, ETLTask};
use crate::parameter_tuple::ParameterTuple;
use crate::python::AST;
use crate::r::preamble::RPreamble;
use crate::r::r_import::RImport;
use crate::r::task::RBasedTask;
use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;
use std::collections::{BTreeSet, HashMap, HashSet};
use tracing::trace;
use uuid::Uuid;

pub struct RBasedCodeBlock<T>
where
    T: ETLFlow<ImportType = RImport>,
{
    tasks_dict: Option<AST>,
    task_identifiers: HashMap<Uuid, AST>,
    tasks: Vec<RBasedTask<T>>,
    params: HashMap<String, Option<ParameterTuple>>,
}
impl<T> CodeBlock<T> for RBasedCodeBlock<T>
where
    T: ETLFlow<ImportType = RImport>,
{
    type P = RPreamble;
    type E = RBasedTask<T>;

    fn construct<'a>(
        tasks_dict: Option<AST>,
        tasks: Vec<Self::E>,
        task_identifiers: HashMap<Uuid, AST>,
        params: HashMap<String, Option<ParameterTuple>>,
    ) -> Self {
        Self {
            tasks_dict,
            tasks,
            task_identifiers,
            params,
        }
    }

    fn get_statements(
        &self,
        endpoints: &EndpointConfig,
    ) -> (Vec<AST>, LinkedHashSet<RPreamble>, BTreeSet<RImport>) {
        let preambles_and_statements = self
            .tasks
            .iter()
            .map(|x| x.get_statements(endpoints))
            .collect::<Vec<_>>();
        // TODO: get_statements should run for members of self.tasks
        let preambles = preambles_and_statements
            .iter()
            .map(|x| x.1.clone().into_iter())
            .flatten()
            .map(|x| RPreamble::new(x))
            .collect::<LinkedHashSet<RPreamble>>();
        let imports = preambles_and_statements
            .iter()
            .map(|x| x.2.clone().into_iter())
            .flatten()
            .collect::<BTreeSet<RImport>>();
        let statements = preambles_and_statements
            .iter()
            .map(|x| x.0.clone())
            .flatten()
            .collect::<Vec<_>>();
        (statements, preambles, imports)
    }
    fn get_tasks_dict(&self) -> Option<AST> {
        self.tasks_dict.clone()
    }

    fn get_identifiers(&self) -> HashMap<Uuid, AST> {
        self.task_identifiers.clone()
    }

    fn get_params(&self) -> HashMap<String, Option<ParameterTuple>> {
        self.params.clone()
    }
}
