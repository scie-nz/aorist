#![allow(dead_code)]
use crate::code::CodeBlockWithDefaultConstructor;
use crate::concept::{Concept, ConceptAncestry};
use crate::constraint::SatisfiableOuterConstraint;
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
use aorist_primitives::{OuterConstraint, TConstraint, TBuilder};
use aorist_primitives::{TAoristObject, TConceptEnum, TConstraintEnum};
use inflector::cases::snakecase::to_snake_case;
use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::marker::PhantomData;
use std::sync::{Arc, RwLock, RwLockReadGuard};
use tracing::{debug, level_enabled, trace, Level};
use uuid::Uuid;

type ConstraintsBlockMap<'a, 'b, C> = LinkedHashMap<
    String,
    (
        LinkedHashSet<String>,
        LinkedHashMap<(Uuid, String), Arc<RwLock<ConstraintState<'a, 'b, C>>>>,
    ),
>;

pub trait Driver<'a, 'b, D, C>
where
    D: FlowBuilderBase,
    D:
        FlowBuilderMaterialize<
            BuilderInputType = <Self::CB as ConstraintBlock<
                'a,
                'b,
                <D as FlowBuilderBase>::T,
                C,
            >>::BuilderInputType,
        >,
    <D as FlowBuilderBase>::T: 'a,
    Self::CB: 'a,
    C: OuterConstraint<'a, 'b> + SatisfiableOuterConstraint<'a, 'b>,
    'a: 'b,
{
    type CB: ConstraintBlock<'a, 'b, <D as FlowBuilderBase>::T, C>;

    fn get_relevant_builders(
        topline_constraint_names: &LinkedHashSet<String>,
    ) -> Vec<<<C as OuterConstraint<'a, 'b>>::TEnum as TConstraintEnum<'a, 'b>>::BuilderT> {
        let mut builders =
            <<C as OuterConstraint<'a, 'b>>::TEnum as TConstraintEnum<'a, 'b>>::builders()
                .into_iter()
                .map(|x| (x.get_constraint_name(), x))
                .collect::<LinkedHashMap<String, _>>();
        let mut builder_q = topline_constraint_names
            .clone()
            .into_iter()
            .map(|x| {
                (
                    x.clone(),
                    builders
                        .remove(&x)
                        .expect(format!("Missing constraint named {}", x).as_str()),
                )
            })
            .collect::<VecDeque<_>>();
        let mut relevant_builders = LinkedHashMap::new();
        let mut visited = HashSet::new();
        let mut g: LinkedHashMap<String, LinkedHashSet<String>> = LinkedHashMap::new();
        let mut rev: HashMap<String, Vec<String>> = HashMap::new();

        while builder_q.len() > 0 {
            let (key, builder) = builder_q.pop_front().unwrap();
            let edges = g.entry(key.clone()).or_insert(LinkedHashSet::new());
            debug!("Constraint {} requires:", key);
            for req in builder.get_required_constraint_names() {
                debug!("  - {}", req);
                if !visited.contains(&req) {
                    let another = builders.remove(&req).unwrap();
                    builder_q.push_back((req.clone(), another));
                    visited.insert(req.clone());
                }
                edges.insert(req.clone());
                let rev_edges = rev.entry(req.clone()).or_insert(Vec::new());
                rev_edges.push(key.clone());
            }
            relevant_builders.insert(key.clone(), builder);
        }
        let mut sorted_builders = Vec::new();
        while g.len() > 0 {
            let leaf = g
                .iter()
                .filter(|(_, v)| v.len() == 0)
                .map(|(k, _)| k)
                .next()
                .unwrap()
                .clone();

            let builder = relevant_builders.remove(&leaf).unwrap();
            if let Some(parents) = rev.remove(&leaf) {
                for parent in parents {
                    g.get_mut(&parent).unwrap().remove(&leaf);
                }
            }
            sorted_builders.push(builder);
            g.remove(&leaf);
        }

        sorted_builders
    }
}
