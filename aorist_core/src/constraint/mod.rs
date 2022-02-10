use crate::parameter_tuple::ParameterTuple;
use abi_stable::external_types::parking_lot::rw_lock::RRwLock;
use abi_stable::std_types::RArc;
use anyhow::Result;
use aorist_primitives::Dialect;
use aorist_primitives::{Ancestry, AoristConceptBase, ToplineConcept};
use aorist_util::{AString, AVec, ATaskId, AOption, AUuid};
use std::collections::{HashMap, HashSet, VecDeque};
use std::marker::PhantomData;
use tracing::info;
use tracing::{debug, level_enabled, trace, Level};
use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;
use abi_stable::sabi_trait;
/*
use abi_stable::{
    declare_root_module_statics,
    library::RootModule,
    package_version_strings,
    sabi_types::VersionStrings,
    std_types::{RResult, RString, RVec},
    StableAbi,
};*/

pub trait SatisfiableConstraint<'a>: TConstraint<'a> {
    type TAncestry: Ancestry;
    fn satisfy(
        &mut self,
        c: <Self::TAncestry as Ancestry>::TConcept,
        d: &Dialect,
        ancestry: RArc<Self::TAncestry>,
    ) -> Result<AOption<(AString, AString, ParameterTuple, Dialect)>>;

    fn satisfy_given_preference_ordering(
        &mut self,
        r: <Self::TAncestry as Ancestry>::TConcept,
        preferences: &AVec<Dialect>,
        ancestry: RArc<Self::TAncestry>,
    ) -> Result<(AString, AString, ParameterTuple, Dialect)>;
}


// TODO: duplicate function, should be unified in trait
pub trait SatisfiableOuterConstraint<'a>: OuterConstraint<'a> {
    fn satisfy_given_preference_ordering(
        &mut self,
        c: <<Self as OuterConstraint<'a>>::TAncestry as Ancestry>::TConcept,
        preferences: &AVec<Dialect>,
        ancestry: RArc<<Self as OuterConstraint<'a>>::TAncestry>,
    ) -> Result<(AString, AString, ParameterTuple, Dialect)>;
}
pub trait TBuilder<'a> {
    type TEnum: ToplineConcept;
    type TAncestry: Ancestry;
    type OuterType: OuterConstraint<'a>; //, TEnum=Self::EnumType>;
                                         //type EnumType: TConstraintEnum<'a, BuilderT=Self>;
    fn builders() -> AVec<Self>
    where
        Self: Sized;
    fn get_constraint_name(&self) -> AString;
    fn get_required_constraint_names(&self) -> AVec<AString>;
    fn build_constraint(
        &self,
        root_uuid: AUuid,
        potential_child_constraints: AVec<RArc<RRwLock<Self::OuterType>>>,
    ) -> Result<Self::OuterType>;
    fn get_root_type_name(&self) -> Result<AString>;
    fn get_required(&self, root: Self::TEnum, ancestry: &Self::TAncestry) -> AVec<AUuid>;
    fn should_add(&self, root: Self::TEnum, ancestry: &Self::TAncestry) -> bool;
    fn attach_constraints(
        &self,
        by_object_type: &HashMap<AString, AVec<Self::TEnum>>,
        family_trees: &HashMap<ATaskId, HashMap<AString, HashSet<AUuid>>>,
        ancestry: &Self::TAncestry,
        generated_constraints: &mut LinkedHashMap<
            AString,
            LinkedHashMap<ATaskId, RArc<RRwLock<Self::OuterType>>>,
        >,
        visited_constraint_names: &mut LinkedHashSet<AString>,
    ) -> Result<()> {
        let root_object_type = self.get_root_type_name()?;
        let constraint_name = self.get_constraint_name();

        if let Some(root_concepts) = by_object_type.get(&root_object_type) {
            debug!(
                "Attaching constraint {} to {} objects of type {}.",
                constraint_name,
                root_concepts.len(),
                root_object_type
            );

            for root in root_concepts.iter() {
                let root_key = ATaskId::new(root.get_uuid(), root.get_type());
                let family_tree = family_trees.get(&root_key).unwrap();
                if self.should_add(root.clone(), &ancestry) {
                    let raw_potential_child_constraints = self
                        .get_required_constraint_names()
                        .into_iter()
                        .map(|req| (req.clone(), generated_constraints.get(&req)))
                        .filter(|(_req, x)| x.is_some())
                        .map(|(req, x)| (req, x.unwrap()))
                        .collect::<AVec<_>>();
                    if level_enabled!(Level::DEBUG) {
                        debug!(
                        "Creating constraint {:?} on root {:?} with potential child constraints:",
                        self.get_constraint_name(),
                        &root_key
                    );
                        for (required_constraint_name, map) in
                            raw_potential_child_constraints.iter()
                        {
                            debug!(" - for {}:", required_constraint_name);
                            for (key, v) in map.iter() {
                                let downstream = v.read();
                                debug!(
                                    " -- {:?}: {:?}",
                                    key,
                                    (downstream.get_uuid(), downstream.get_name())
                                );
                            }
                        }
                    }
                    let other_required_concept_uuids = self
                        .get_required(root.clone(), &ancestry)
                        .into_iter()
                        .collect::<HashSet<_>>();
                    if other_required_concept_uuids.len() > 0 {
                        trace!(
                            "Found {} other required concept uuids for root {:?}",
                            other_required_concept_uuids.len(),
                            root.get_uuid()
                        );
                    }
                    let potential_child_constraints = raw_potential_child_constraints
                        .into_iter()
                        .map(|(_req, x)| {
                            x.iter()
                                .filter(
                                    |(potential_root, _constraint)| {
                                        let potential_root_id = (*potential_root).get_constraint_id();
                                        let potential_root_type = (*potential_root).get_root_type();
                                        (match family_tree.get(&potential_root_type) {
                                            None => false,
                                            Some(set) => set.contains(&potential_root_id),
                                        } || other_required_concept_uuids
                                            .contains(&potential_root_id))
                                    },
                                )
                                .map(|(_, constraint)| constraint.clone())
                        })
                        .flatten()
                        .collect::<AVec<RArc<RRwLock<Self::OuterType>>>>();
                    if level_enabled!(Level::DEBUG) {
                        debug!("After filtering:",);
                        for downstream_rw in potential_child_constraints.iter() {
                            let downstream = downstream_rw.read();
                            debug!(" --  {:?}", (downstream.get_uuid(), downstream.get_name()));
                        }
                    }
                    let constraint =
                        self.build_constraint(root.get_uuid(), potential_child_constraints)?;
                    let gen_for_constraint = generated_constraints
                        .entry(constraint_name.clone())
                        .or_insert(LinkedHashMap::new());
                    assert!(!gen_for_constraint.contains_key(&root_key));
                    if level_enabled!(Level::DEBUG) {
                        debug!(
                            "Added constraint {:?} on root {:?} with the following dependencies:",
                            (constraint.get_uuid(), constraint.get_name()),
                            &root_key
                        );
                        //for downstream_rw in constraint.get_downstream_constraints()? {
                        //    let downstream = downstream_rw.read();
                        //    debug!(" --  {:?}", (downstream.get_uuid()?, downstream.get_name()));
                        //}
                    }
                    gen_for_constraint.insert(root_key, RArc::new(RRwLock::new(constraint)));
                } else {
                    debug!("Constraint was filtered out.");
                }
            }
        } else {
            debug!(
                "Found no concepts of type {} for {}",
                root_object_type, constraint_name,
            );
        }
        for req in self.get_required_constraint_names() {
            assert!(visited_constraint_names.contains(&req));
        }
        visited_constraint_names.insert(constraint_name.clone());
        Ok(())
    }
    fn get_relevant_builders(topline_constraint_names: &LinkedHashSet<AString>) -> AVec<Self> 
    where Self: Sized {
        let mut visited = HashSet::new();
        let mut relevant_builders = LinkedHashMap::new();
        let mut g: LinkedHashMap<AString, LinkedHashSet<AString>> = LinkedHashMap::new();
        let mut rev: HashMap<AString, AVec<AString>> = HashMap::new();

        for start in topline_constraint_names {
            let mut builders = Self::builders()
                .into_iter()
                .map(|x| (x.get_constraint_name(), x))
                .collect::<LinkedHashMap<AString, _>>();

            let constraint = builders
                .remove(start)
                .expect(format!("Missing constraint named {}", start).as_str());
            let mut builder_q = vec![(start.clone(), constraint)]
                .into_iter()
                .collect::<VecDeque<_>>();

            while builder_q.len() > 0 {
                let (key, builder) = builder_q.pop_front().unwrap();
                let edges = g.entry(key.clone()).or_insert(LinkedHashSet::new());
                debug!("Constraint {} requires:", key);
                for req in builder.get_required_constraint_names() {
                    debug!("  - {}", req);
                    if !visited.contains(&req) {
                        let another = match builders.remove(&req) {
                            Some(x) => x,
                            None => panic!("Cannot find {} in builders.", req),
                        };
                        builder_q.push_back((req.clone(), another));
                        visited.insert(req.clone());
                    }
                    edges.insert(req.clone());
                    let rev_edges = rev.entry(req.clone()).or_insert(AVec::new());
                    rev_edges.push(key.clone());
                }
                relevant_builders.insert(key.clone(), builder);
            }
        }
        let mut sorted_builders = AVec::new();
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

pub trait TConstraintEnum<'a>: Sized + Clone {
    fn get_required_constraint_names() -> HashMap<AString, AVec<AString>>;
    fn get_explanations() -> HashMap<AString, (AOption<AString>, AOption<AString>)>;
    #[cfg(feature = "python")]
    fn get_py_obj<'b>(&self, py: pyo3::Python<'b>) -> pyo3::prelude::PyObject;
}

#[sabi_trait]
pub trait OuterConstraint<'a>: std::fmt::Display + Clone {
    type TEnum: Sized + TConstraintEnum<'a>;
    type TAncestry: Ancestry;
    fn get_task_id(&self) -> ATaskId {
       ATaskId::new(self.get_uuid(), self.get_root_type_name())
    }

    fn get_name(&self) -> AString;
    fn get_uuid(&self) -> AUuid;
    fn get_root(&self) -> AString;
    fn get_root_uuid(&self) -> AUuid;
    //fn get_downstream_constraints(&self) -> Result<AVec<RArc<RRwLock<Self>>>>;
    fn get_dependencies(&self) -> AVec<ATaskId>;
    fn requires_program(&self) -> bool;
    fn get_root_type_name(&self) -> AString;
    fn inner(&self, caller: AString) -> &Self::TEnum;
}
pub trait TConstraint<'a>
where
    Self::Root: AoristConceptBase,
    Self::Outer: OuterConstraint<'a, TAncestry = Self::Ancestry>,
    Self::Ancestry: Ancestry,
{
    type Root;
    type Outer;
    type Ancestry;

    fn get_root_type_name() -> Result<AString>;
    fn get_required_constraint_names() -> AVec<AString>;
    fn new(
        root_uuid: AUuid,
        potential_child_constraints: AVec<RArc<RRwLock<Self::Outer>>>,
    ) -> Result<Self>
    where
        Self: Sized;
    fn should_add(
        root: <<Self as TConstraint<'a>>::Ancestry as Ancestry>::TConcept,
        ancestry: &<Self as TConstraint<'a>>::Ancestry,
    ) -> bool;
}
pub trait ConstraintSatisfactionBase<'a>
where
    Self::RootType: AoristConceptBase,
    Self::Outer: OuterConstraint<'a>,
    Self::ConstraintType: TConstraint<'a, Root = Self::RootType, Outer = Self::Outer>,
{
    type ConstraintType;
    type RootType;
    type Outer;
}
pub struct ConstraintBuilder<'a, T: TConstraint<'a>> {
    pub _phantom: PhantomData<T>,
    pub _phantom_lt: PhantomData<&'a ()>,
}
impl<'a, T: TConstraint<'a>> ConstraintBuilder<'a, T> {
    pub fn build_constraint(
        &self,
        root_uuid: AUuid,
        potential_child_constraints: AVec<RArc<RRwLock<T::Outer>>>,
    ) -> Result<T> {
        <T as crate::constraint::TConstraint<'a>>::new(root_uuid, potential_child_constraints)
    }
    pub fn get_root_type_name(&self) -> Result<AString> {
        <T as crate::constraint::TConstraint<'a>>::get_root_type_name()
    }
}
