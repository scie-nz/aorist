use crate::flow::etl_flow::ETLFlow;
use crate::flow::flow_builder_input::FlowBuilderInput;
use std::error::Error;

pub trait FlowBuilderBase
where
    Self: Sized,
{
    type T: ETLFlow;
    fn new() -> Self;
}
pub trait FlowBuilderMaterialize
where
    Self: Sized,
    Self::BuilderInputType: FlowBuilderInput,
    Self::ErrorType: Error + Send + Sync + 'static,
{
    type BuilderInputType;
    type ErrorType;

    fn materialize(
        &self,
        statements_and_preambles: Vec<Self::BuilderInputType>,
    ) -> Result<String, Self::ErrorType>;
}
