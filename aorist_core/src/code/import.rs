use std::hash::Hash;

pub trait Import: Hash + Eq + Ord + Clone {}
