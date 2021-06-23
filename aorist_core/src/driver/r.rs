#![allow(dead_code)]
use crate::concept::{Concept, ConceptAncestry};
use crate::constraint::SatisfiableOuterConstraint;
use crate::constraint_state::ConstraintState;
use crate::dialect::{Bash, Dialect, Python, R};
use crate::driver::{ConstraintsBlockMap, Driver};
use crate::endpoints::EndpointConfig;
use crate::flow::{ETLFlow, FlowBuilderBase, FlowBuilderMaterialize};
use crate::r::{RBasedConstraintBlock, RFlowBuilderInput, RImport, RPreamble};
use anyhow::Result;
use aorist_ast::AncestorRecord;
use aorist_primitives::OuterConstraint;
use aorist_primitives::TConstraintEnum;
use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

pub struct RBasedDriver<'a, 'b, D, C>
where
    D: FlowBuilderBase,
    <D as FlowBuilderBase>::T: ETLFlow<ImportType = RImport, PreambleType = RPreamble> + 'a,
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
impl<'a, 'b, D, C> Driver<'a, 'b, D, C> for RBasedDriver<'a, 'b, D, C>
where
    'a: 'b,
    D: FlowBuilderBase,
    D: FlowBuilderMaterialize<BuilderInputType = RFlowBuilderInput>,
    <D as FlowBuilderBase>::T: ETLFlow<ImportType = RImport, PreambleType = RPreamble> + 'a,
    C: OuterConstraint<'a, 'b, TAncestry = ConceptAncestry<'a>>
        + SatisfiableOuterConstraint<'a, 'b>
        + 'b,
{
    type CB = RBasedConstraintBlock<'a, 'b, <D as FlowBuilderBase>::T, C>;

    fn get_constraint_rwlock(&self, uuid: &(Uuid, String)) -> Arc<RwLock<C>> {
        self.constraints.get(uuid).unwrap().clone()
    }
    fn get_preferences(&self) -> Vec<Dialect> {
        vec![
            Dialect::R(R {}),
            Dialect::Python(Python::new(vec![])),
            Dialect::Bash(Bash {}),
        ]
    }

    fn get_endpoints(&'b self) -> &'b EndpointConfig {
        &self.endpoints
    }

    fn get_ancestry(&self) -> Arc<ConceptAncestry<'a>> {
        self.ancestry.clone()
    }
    fn mark_constraint_state_as_satisfied(
        &mut self,
        id: (Uuid, String),
        state: Arc<RwLock<ConstraintState<'a, 'b, C>>>,
    ) {
        self.satisfied_constraints.insert(id, state.clone());
    }
    fn init_unsatisfied_constraints(&self) -> Result<ConstraintsBlockMap<'a, 'b, C>> {
        Self::get_unsatisfied_constraints(
            &self.constraints,
            self.concepts.clone(),
            &self.ancestors,
            self.topline_constraint_names.clone(),
        )
    }
    fn add_block(
        &mut self,
        constraint_block: RBasedConstraintBlock<'a, 'b, <D as FlowBuilderBase>::T, C>,
    ) {
        self.blocks.push(constraint_block);
    }
    fn get_constraint_explanation(
        &self,
        constraint_name: &String,
    ) -> (Option<String>, Option<String>) {
        self.constraint_explanations
            .get(constraint_name)
            .unwrap()
            .clone()
    }
    fn get_blocks(&'b self) -> &'b Vec<Self::CB> {
        &self.blocks
    }
    fn get_dependencies(&self) -> Vec<String> {
        // TODO: add libraries
        vec![]
    }
    fn _new(
        concepts: Arc<RwLock<HashMap<(Uuid, String), Concept<'a>>>>,
        constraints: LinkedHashMap<(Uuid, String), Arc<RwLock<C>>>,
        ancestry: Arc<ConceptAncestry<'a>>,
        endpoints: EndpointConfig,
        ancestors: HashMap<(Uuid, String), Vec<AncestorRecord>>,
        topline_constraint_names: LinkedHashSet<String>,
    ) -> Self {
        Self {
            concepts,
            constraints,
            satisfied_constraints: HashMap::new(),
            blocks: Vec::new(),
            ancestry,
            dag_type: PhantomData,
            endpoints,
            constraint_explanations: <<C as OuterConstraint<'a, 'b>>::TEnum as TConstraintEnum<
                'a,
                'b,
            >>::get_explanations(),
            ancestors,
            topline_constraint_names,
            _lt_phantom: PhantomData,
        }
    }
}
