use crate::endpoints::EndpointConfig;
use crate::flow::{CompressibleETLTask, ETLFlow, ETLTask, TaskBase};
use crate::r::preamble::RPreamble;
use crate::r::r_import::RImport;
use aorist_ast::AST;

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
    T: ETLFlow<ImportType = RImport, PreambleType = RPreamble>,
{
    StandaloneRBasedTask(StandaloneRBasedTask<T>),
    ForLoopRBasedTask(ForLoopRBasedTask<T>),
}
impl<T> ETLTask<T> for RBasedTask<T>
where
    T: ETLFlow<ImportType = RImport, PreambleType = RPreamble>,
{
    type S = StandaloneRBasedTask<T>;
    fn standalone_task(task: Self::S) -> Self {
        Self::StandaloneRBasedTask(task)
    }
}
impl<T> TaskBase<T> for RBasedTask<T> where
    T: ETLFlow<ImportType = RImport, PreambleType = RPreamble>
{
}
impl<T> RBasedTask<T>
where
    T: ETLFlow<ImportType = RImport, PreambleType = RPreamble>,
{
    pub fn get_statements(
        &self,
        endpoints: &EndpointConfig,
    ) -> (Vec<AST>, Vec<RPreamble>, Vec<RImport>) {
        match &self {
            RBasedTask::StandaloneRBasedTask(x) => x.get_statements(endpoints),
            RBasedTask::ForLoopRBasedTask(x) => x.get_statements(endpoints),
        }
    }
}
impl<T> CompressibleETLTask<T> for RBasedTask<T>
where
    T: ETLFlow<ImportType = RImport, PreambleType = RPreamble>,
{
    type F = ForLoopRBasedTask<T>;
}
