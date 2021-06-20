#![allow(dead_code)]
use crate::code::CodeBlockWithDefaultConstructor;
use crate::concept::{Concept, ConceptAncestry};
use crate::constraint::TConstraint;
use aorist_primitives::{OuterConstraint};
use crate::constraint::{SatisfiableOuterConstraint};
use crate::constraint_block::ConstraintBlock;
use crate::constraint_state::ConstraintState;
use crate::dialect::{Bash, Dialect, Presto, Python, R};
use crate::endpoints::EndpointConfig;
use crate::flow::{ETLFlow, FlowBuilderBase, FlowBuilderMaterialize};
use crate::parameter_tuple::ParameterTuple;
#[cfg(feature = "python")]
use crate::python::{
    PythonBasedConstraintBlock, PythonFlowBuilderInput, PythonImport, PythonPreamble,
};
#[cfg(feature = "r")]
use crate::r::{RBasedConstraintBlock, RFlowBuilderInput, RImport, RPreamble};
use crate::universe::Universe;
use anyhow::Result;
use aorist_ast::{AncestorRecord, SimpleIdentifier, AST};
use aorist_primitives::{TConceptEnum, TAoristObject};
use inflector::cases::snakecase::to_snake_case;
use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::marker::PhantomData;
use std::sync::{Arc, RwLock, RwLockReadGuard};
use tracing::{debug, level_enabled, trace, Level};
use uuid::Uuid;

type ConstraintsBlockMap<'a, C> =
    LinkedHashMap<
        String,
        (
            LinkedHashSet<String>,
            LinkedHashMap<(Uuid, String), Arc<RwLock<ConstraintState<'a, C>>>>,
        ),
    >;

pub trait Driver<'a, 'b, D, C>
where
    D: FlowBuilderBase,
    D: FlowBuilderMaterialize<
        BuilderInputType=<Self::CB as ConstraintBlock<
            'b, <D as FlowBuilderBase>::T, C
        >>::BuilderInputType
    >,
    <D as FlowBuilderBase>::T: 'a,
    Self::CB: 'a,
    C: OuterConstraint<'b> + SatisfiableOuterConstraint<'b>,
    'a: 'b,
{
    type CB: ConstraintBlock<'b, <D as FlowBuilderBase>::T, C>;
}
