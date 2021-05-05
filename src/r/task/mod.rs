use crate::endpoints::EndpointConfig;
use crate::flow::{ETLFlow, TaskBase, ETLTask};
use crate::python::{AST};
use crate::r::r_import::RImport;

mod compressed;
mod key;
mod standalone;
mod uncompressible;

pub use compressed::*;
pub use key::*;
pub use standalone::*;
pub use uncompressible::*;

pub enum RBasedTask<T>
where
    T: ETLFlow,
{
    StandaloneRBasedTask(StandaloneRBasedTask<T>),
}
impl<T> ETLTask<T> for RBasedTask<T>
where
    T: ETLFlow,
{
    type S = StandaloneRBasedTask<T>;
    fn standalone_task(task: Self::S) -> Self {
        Self::StandaloneRBasedTask(task)
    }
}
impl<T> TaskBase<T> for RBasedTask<T> where T: ETLFlow {}
impl<T> RBasedTask<T>
where T: ETLFlow<ImportType=RImport> {
    pub fn get_statements(
        &self,
        endpoints: &EndpointConfig,
    ) -> (Vec<AST>, Vec<String>, Vec<RImport>) {
        match &self {
            RBasedTask::StandaloneRBasedTask(x) => x.get_statements(endpoints),
        }
    }
}
