use crate::endpoints::*;
use abi_stable::external_types::parking_lot::rw_lock::RRwLock;
use abi_stable::std_types::{RArc, ROption};
use abi_stable::StableAbi;
use aorist_util::{AOption, AString, AUuid, AVec, AoristRef, ATaskId};
use serde::Serialize;
use siphasher::sip128::{Hasher128, SipHasher};
use std::collections::{BTreeSet, HashMap};
use std::fmt::Debug;
use std::hash::Hasher;

pub trait ConceptEnum {
    fn uuid(&self) -> AOption<AUuid>;
}

pub trait AoristConceptBase: Clone + Debug + Serialize + PartialEq + StableAbi {
    type TChildrenEnum: ConceptEnum;
    fn get_uuid(&self) -> AOption<AUuid>;
    fn set_uuid(&mut self, uuid: AUuid);
    fn get_tag(&self) -> AOption<AString>;
    fn compute_uuids(&mut self);
    fn deep_clone(&self) -> Self;
    fn get_children(
        &self,
    ) -> AVec<(
        // struct name
        AString,
        // field name
        AOption<AString>,
        // ix
        AOption<usize>,
        // uuid
        AOption<AUuid>,
        // wrapped reference
        Self::TChildrenEnum,
    )>;
}

pub trait AoristConcept {
    type TChildrenEnum: ConceptEnum;
    fn get_uuid(&self) -> AOption<AUuid>;
    fn set_uuid(&mut self, uuid: AUuid);
    fn get_tag(&self) -> AOption<AString>;
    fn compute_uuids(&mut self);
    fn get_children_uuid(&self) -> AVec<AUuid>;
    fn get_uuid_from_children_uuid(&self) -> AUuid {
        let child_uuids = self.get_children_uuid();
        if child_uuids.len() > 0 {
            let uuids = child_uuids.into_iter().collect::<BTreeSet<AUuid>>();
            let mut hasher = SipHasher::new();
            for uuid in uuids {
                hasher.write(&uuid.as_bytes());
            }
            let bytes: [u8; 16] = hasher.finish128().as_bytes();
            AUuid::from_bytes(bytes)
        } else {
            // TODO: this should just be created from the hash
            AUuid::new_v4()
        }
    }
    fn get_children(
        &self,
    ) -> AVec<(
        // struct name
        AString,
        // field name
        AOption<AString>,
        // ix
        AOption<usize>,
        // uuid
        AOption<AUuid>,
        // wrapped reference
        Self::TChildrenEnum,
    )>;
    fn deep_clone(&self) -> Self;
}

pub trait ToplineConcept: Sized + Clone + StableAbi {
    type TUniverse: AoristConcept + AoristUniverse;
    fn get_parent_id(&self) -> AOption<ATaskId>;
    fn get_type(&self) -> AString;
    fn get_uuid(&self) -> AUuid;
    fn get_tag(&self) -> AOption<AString>;
    fn get_index_as_child(&self) -> usize;
    fn get_child_concepts(&self) -> AVec<Self>;
    fn populate_child_concept_map(&self, concept_map: &mut HashMap<ATaskId, Self>);
    fn from_universe(universe: Self::TUniverse) -> Self;
}
pub trait ToplineConceptBase: Sized + Clone + Debug + Serialize + PartialEq + StableAbi {
    type TUniverse: AoristConcept + AoristUniverse;
    fn get_parent_id(&self) -> AOption<ATaskId>;
    fn get_type(&self) -> AString;
    fn get_index_as_child(&self) -> usize;
    fn get_child_concepts(&self) -> AVec<AoristRef<Self>>;
    fn populate_child_concept_map(
        &self,
        concept_map: &mut HashMap<ATaskId, AoristRef<Self>>,
    );
    fn build_universe(universe: Self::TUniverse) -> Self;
}

pub trait AoristUniverse {
    type TEndpoints: Clone;
    fn get_endpoints(&self) -> Self::TEndpoints;
}
pub trait TPrestoEndpoints {
    fn presto_config(&self) -> PrestoConfig;
}
pub trait Ancestry {
    type TConcept: ConceptEnum + Clone + ToplineConcept;
    fn new(parents: RArc<RRwLock<HashMap<ATaskId, Self::TConcept>>>) -> Self;
    fn get_parents(&self) -> RArc<RRwLock<HashMap<ATaskId, Self::TConcept>>>;
}
impl<T: PartialEq + Serialize + Debug + Clone + AoristConceptBase + StableAbi> AoristConcept
    for AoristRef<T>
{
    type TChildrenEnum = <T as AoristConceptBase>::TChildrenEnum;
    fn get_uuid(&self) -> AOption<AUuid> {
        self.0.read().get_uuid()
    }
    fn deep_clone(&self) -> Self {
        AoristRef(RArc::new(RRwLock::new(self.0.read().deep_clone())))
    }
    fn set_uuid(&mut self, uuid: AUuid) {
        self.0.write().set_uuid(uuid);
    }
    fn compute_uuids(&mut self) {
        self.0.write().compute_uuids();
        let uuid;
        uuid = self.get_uuid_from_children_uuid();
        self.0.write().set_uuid(uuid);
    }
    fn get_children_uuid(&self) -> AVec<AUuid> {
        self.get_children()
            .iter()
            .map(|x| x.4.uuid().unwrap())
            .collect()
    }
    fn get_tag(&self) -> AOption<AString> {
        self.0.read().get_tag()
    }
    fn get_children(
        &self,
    ) -> AVec<(
        // struct name
        AString,
        // field name
        AOption<AString>,
        // ix
        AOption<usize>,
        // uuid
        AOption<AUuid>,
        Self::TChildrenEnum,
    )> {
        self.0.read().get_children()
    }
}

impl<T: Clone + Debug + Serialize + PartialEq + AoristConceptBase + StableAbi> ConceptEnum
    for AoristRef<T>
{
    fn uuid(&self) -> AOption<AUuid> {
        self.0.read().get_uuid()
    }
}

impl<
        T: Debug + Clone + Serialize + PartialEq + ToplineConceptBase + AoristConceptBase + StableAbi,
    > ToplineConcept for AoristRef<T>
{
    type TUniverse = <T as ToplineConceptBase>::TUniverse;
    fn get_parent_id(&self) -> AOption<ATaskId> {
        self.0.read().get_parent_id()
    }
    fn from_universe(universe: Self::TUniverse) -> Self {
        AoristRef(RArc::new(RRwLock::new(T::build_universe(universe))))
    }
    fn get_type(&self) -> AString {
        self.0.read().get_type()
    }
    fn get_uuid(&self) -> AUuid {
        self.0.read().get_uuid().unwrap()
    }
    fn get_tag(&self) -> AOption<AString> {
        self.0.read().get_tag()
    }
    fn get_index_as_child(&self) -> usize {
        self.0.read().get_index_as_child()
    }
    fn get_child_concepts(&self) -> AVec<Self> {
        self.0.read().get_child_concepts()
    }
    fn populate_child_concept_map(&self, concept_map: &mut HashMap<ATaskId, Self>) {
        self.0.read().populate_child_concept_map(concept_map)
    }
}
// note: both Universe and EndpointConfig must exist
impl<T: Debug + Clone + Serialize + PartialEq + AoristUniverseBase + StableAbi> AoristUniverse
    for AoristRef<T>
{
    type TEndpoints = <T as AoristUniverseBase>::TEndpoints;
    fn get_endpoints(&self) -> Self::TEndpoints {
        (*self.0.read()).get_endpoints()
    }
}
pub trait AoristUniverseBase {
    type TEndpoints: Clone;
    fn get_endpoints(&self) -> Self::TEndpoints;
}
#[repr(C)]
#[derive(Clone, Serialize, Debug, StableAbi, PartialEq, Eq)]
pub struct AConcept<T: Debug + Clone + Serialize + PartialEq + StableAbi + AoristConceptBase + Eq> {
    obj_ref: AoristRef<T>,
    index_as_child: usize,
    parent_uuid: AOption<AUuid>,
    parent_id: AOption<AString>,
}
impl<T: Debug + Clone + Serialize + PartialEq + StableAbi + AoristConceptBase + Eq> AConcept<T> {
    pub fn new(obj_ref: AoristRef<T>, ix: usize, id: AOption<(AUuid, AString)>) -> Self {
        Self {
            obj_ref,
            index_as_child: ix,
            parent_uuid: id.clone().and_then(|x| ROption::RSome(x.0)),
            parent_id: id.and_then(|x| ROption::RSome(x.1)),
        }
    }
    pub fn get_index_as_child(&self) -> usize {
        self.index_as_child
    }
    pub fn get_parent_id(&self) -> AOption<ATaskId> {
        if let ROption::RSome(ref uuid) = self.parent_uuid.0 {
            if let ROption::RSome(ref id) = self.parent_id.0 {
                return AOption(ROption::RSome(ATaskId::new(uuid.clone(), id.clone())));
            } else {
                panic!("Id was None when uuid was Some(_)");
            }
        }
        AOption(ROption::RNone)
    }
    pub fn get_reference(&self) -> AoristRef<T> {
        self.obj_ref.clone()
    }
    pub fn get_own_uuid(&self) -> AOption<AUuid> {
        self.obj_ref.0.read().get_uuid()
    }
    pub fn set_uuid(&mut self, uuid: AUuid) {
        self.obj_ref.0.write().set_uuid(uuid);
    }
    pub fn get_uuid(&self) -> AOption<AUuid> {
        self.obj_ref.0.read().get_uuid()
    }
    pub fn deep_clone(&self) -> Self {
        Self {
            obj_ref: AoristRef(RArc::new(RRwLock::new(self.obj_ref.0.read().deep_clone()))),
            index_as_child: self.index_as_child.clone(),
            parent_uuid: self.parent_uuid.clone(),
            parent_id: self.parent_id.clone(),
        }
    }
    pub fn get_tag(&self) -> AOption<AString> {
        self.obj_ref.0.read().get_tag()
    }
    pub fn compute_uuids(&mut self) {
        self.obj_ref.0.write().compute_uuids()
    }
}
