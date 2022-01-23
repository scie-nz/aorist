mod compressed;
mod key;
mod standalone;
mod uncompressible;

pub use compressed::*;
pub use standalone::*;

use crate::flow::{CompressibleETLTask, ETLFlow, ETLTask, TaskBase};
use crate::python::{PythonImport, PythonPreamble, AST};
use aorist_primitives::AoristUniverse;
use aorist_util::AVec;

pub enum PythonBasedTask<T, U>
where
    T: ETLFlow<U, ImportType = PythonImport, PreambleType = PythonPreamble>,
    U: AoristUniverse,
{
    StandalonePythonBasedTask(StandalonePythonBasedTask<T, U>),
    ForLoopPythonBasedTask(ForLoopPythonBasedTask<T, U>),
}
impl<T, U> ETLTask<T, U> for PythonBasedTask<T, U>
where
    T: ETLFlow<U, ImportType = PythonImport, PreambleType = PythonPreamble>,
    U: AoristUniverse,
{
    type S = StandalonePythonBasedTask<T, U>;
    fn standalone_task(task: Self::S) -> Self {
        Self::StandalonePythonBasedTask(task)
    }
}
impl<T, U> CompressibleETLTask<T, U> for PythonBasedTask<T, U>
where
    T: ETLFlow<U, ImportType = PythonImport, PreambleType = PythonPreamble>,
    U: AoristUniverse,
{
    type F = ForLoopPythonBasedTask<T, U>;
}
impl<T, U> PythonBasedTask<T, U>
where
    T: ETLFlow<U, ImportType = PythonImport, PreambleType = PythonPreamble>,
    U: AoristUniverse,
{
    pub fn get_statements(
        &self,
        endpoints: U::TEndpoints,
    ) -> (AVec<AST>, AVec<PythonPreamble>, AVec<PythonImport>) {
        match &self {
            PythonBasedTask::StandalonePythonBasedTask(x) => x.get_statements(endpoints),
            PythonBasedTask::ForLoopPythonBasedTask(x) => x.get_statements(endpoints),
        }
    }
    #[allow(dead_code)]
    fn for_loop_task(task: ForLoopPythonBasedTask<T, U>) -> Self {
        Self::ForLoopPythonBasedTask(task)
    }
}
impl<T, U> TaskBase<T, U> for PythonBasedTask<T, U>
where
    T: ETLFlow<U, ImportType = PythonImport, PreambleType = PythonPreamble>,
    U: AoristUniverse,
{
}
