use crate::endpoints::*;
use siphasher::sip128::{Hasher128, SipHasher};
use std::collections::{BTreeSet, HashMap};
use std::hash::Hasher;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

pub trait ConceptEnum {}
pub trait AoristConcept {
    type TChildrenEnum: ConceptEnum;
    fn get_uuid(&self) -> Option<Uuid>;
    fn get_tag(&self) -> Option<String>;
    fn compute_uuids(&self);
    fn get_children_uuid(&self) -> Vec<Uuid>;
    fn get_uuid_from_children_uuid(&self) -> Uuid {
        let child_uuids = self.get_children_uuid();
        if child_uuids.len() > 0 {
            let uuids = child_uuids.into_iter().collect::<BTreeSet<Uuid>>();
            let mut hasher = SipHasher::new();
            for uuid in uuids {
                hasher.write(uuid.as_bytes());
            }
            let bytes: [u8; 16] = hasher.finish128().as_bytes();
            Uuid::from_bytes(bytes)
        } else {
            // TODO: this should just be created from the hash
            Uuid::new_v4()
        }
    }
    fn get_children(
        &self,
    ) -> Vec<(
        // struct name
        &str,
        // field name
        Option<&str>,
        // ix
        Option<usize>,
        // uuid
        Option<Uuid>,
        // wrapped reference
        Self::TChildrenEnum,
    )>;
}

pub trait TConceptEnum: Sized + Clone {
    type TUniverse: AoristConcept + AoristUniverse;
    fn get_parent_id(&self) -> Option<(Uuid, String)>;
    fn get_type(&self) -> String;
    fn get_uuid(&self) -> Uuid;
    fn get_tag(&self) -> Option<String>;
    fn get_index_as_child(&self) -> usize;
    fn get_child_concepts(&self) -> Vec<Self>;
    fn populate_child_concept_map(&self, concept_map: &mut HashMap<(Uuid, String), Self>);
    fn from_universe(universe: Self::TUniverse) -> Self;
}

pub trait AoristUniverse {
    type TEndpoints: Clone;
    fn get_endpoints(&self) -> Self::TEndpoints;
}
pub trait TPrestoEndpoints {
    fn presto_config(&self) -> PrestoConfig;
}
pub trait Ancestry {
    type TConcept: ConceptEnum + Clone + TConceptEnum;
    fn new(parents: Arc<RwLock<HashMap<(Uuid, String), Self::TConcept>>>) -> Self;
}
pub trait TAoristObject {
    fn get_name(&self) -> &String;
}
