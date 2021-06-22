mod compressed;
mod key;
mod standalone;
mod uncompressible;

pub use compressed::*;
pub use standalone::*;

use crate::endpoints::EndpointConfig;
use crate::flow::{CompressibleETLTask, ETLFlow, ETLTask, TaskBase};
use crate::python::{PythonImport, PythonPreamble, AST};
use crate::concept::AoristRef;

pub enum PythonBasedTask<T>
where
    T: ETLFlow<ImportType = PythonImport, PreambleType = PythonPreamble>,
{
    StandalonePythonBasedTask(StandalonePythonBasedTask<T>),
    ForLoopPythonBasedTask(ForLoopPythonBasedTask<T>),
}
impl<T> ETLTask<T> for PythonBasedTask<T>
where
    T: ETLFlow<ImportType = PythonImport, PreambleType = PythonPreamble>,
{
    type S = StandalonePythonBasedTask<T>;
    fn standalone_task(task: Self::S) -> Self {
        Self::StandalonePythonBasedTask(task)
    }
}
impl<T> CompressibleETLTask<T> for PythonBasedTask<T>
where
    T: ETLFlow<ImportType = PythonImport, PreambleType = PythonPreamble>,
{
    type F = ForLoopPythonBasedTask<T>;
}
impl<T> PythonBasedTask<T>
where
    T: ETLFlow<ImportType = PythonImport, PreambleType = PythonPreamble>,
{
    pub fn get_statements(
        &self,
        endpoints: AoristRef<EndpointConfig>,
    ) -> (Vec<AST>, Vec<PythonPreamble>, Vec<PythonImport>) {
        match &self {
            PythonBasedTask::StandalonePythonBasedTask(x) => x.get_statements(endpoints),
            PythonBasedTask::ForLoopPythonBasedTask(x) => x.get_statements(endpoints),
        }
    }
    #[allow(dead_code)]
    fn for_loop_task(task: ForLoopPythonBasedTask<T>) -> Self {
        Self::ForLoopPythonBasedTask(task)
    }
}
impl<T> TaskBase<T> for PythonBasedTask<T> where
    T: ETLFlow<ImportType = PythonImport, PreambleType = PythonPreamble>
{
}
