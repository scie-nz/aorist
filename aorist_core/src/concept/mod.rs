use siphasher::sip128::{Hasher128, SipHasher};
use std::collections::BTreeSet;
use std::hash::Hasher;
use uuid::Uuid;
use std::marker::PhantomData;

pub trait ConceptEnum<'a> {}
pub trait Ancestry<'a> {
    type TConcept: ConceptEnum<'a>;
}
pub struct WrappedConcept<'a, T> where T: ConceptEnum<'a> {
    _phantom: PhantomData<T>,
    _phantom_lt: PhantomData<&'a ()>,
}
pub trait AoristConcept<'a> {
    
    type TChildrenEnum: ConceptEnum<'a>;

    fn get_children(&'a self) -> Vec<(
        // struct name
        &str,
        // field name
        Option<&str>,
        // ix
        Option<usize>,
        // uuid
        Uuid,
        // wrapped reference
        Self::TChildrenEnum,
    )>;
    fn get_uuid(&self) -> Uuid;
    fn get_children_uuid(&self) -> Vec<Uuid>;
    fn get_tag(&self) -> Option<String>;

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
    fn compute_uuids(&mut self);
}
