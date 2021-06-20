#![allow(dead_code)]
use crate::concept::{Concept, ConceptAncestry};
use crate::constraint::SatisfiableOuterConstraint;
use crate::constraint_state::ConstraintState;
use crate::dialect::{Bash, Dialect, Presto, Python};
use crate::driver::{ConstraintsBlockMap, Driver};
use crate::endpoints::EndpointConfig;
use crate::flow::{ETLFlow, FlowBuilderBase, FlowBuilderMaterialize};
#[cfg(feature = "python")]
use crate::python::{
    PythonBasedConstraintBlock, PythonFlowBuilderInput, PythonImport, PythonPreamble,
};
#[cfg(feature = "r")]
use crate::r::{RBasedConstraintBlock, RFlowBuilderInput, RImport, RPreamble};
use anyhow::Result;
use aorist_ast::AncestorRecord;
use aorist_primitives::OuterConstraint;
use aorist_primitives::TConstraintEnum;
use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;
use std::collections::{HashMap};
use std::marker::PhantomData;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

pub struct RBasedDriver<'a, 'b, D, C>
where
    D: FlowBuilderBase,
    <D as FlowBuilderBase>::T:
        ETLFlow<ImportType = RImport, PreambleType = RPreamble> + 'a,
    C: OuterConstraint<'a, 'b, TAncestry = ConceptAncestry<'a>>
        + SatisfiableOuterConstraint<'a, 'b>
        + 'b,
    'a: 'b,
{
    pub concepts: Arc<RwLock<HashMap<(Uuid, String), Concept<'a>>>>,
    constraints: LinkedHashMap<(Uuid, String), Arc<RwLock<C>>>,
    satisfied_constraints: HashMap<(Uuid, String), Arc<RwLock<ConstraintState<'a, 'b, C>>>>,
    blocks: Vec<RBasedConstraintBlock<'a, 'b, D::T, C>>,
    ancestry: Arc<ConceptAncestry<'a>>,
    dag_type: PhantomData<D>,
    endpoints: EndpointConfig,
    constraint_explanations: HashMap<String, (Option<String>, Option<String>)>,
    ancestors: HashMap<(Uuid, String), Vec<AncestorRecord>>,
    topline_constraint_names: LinkedHashSet<String>,
    _lt_phantom: PhantomData<&'b ()>,
}
