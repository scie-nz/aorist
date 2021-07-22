use linked_hash_set::LinkedHashSet;
use std::collections::{BTreeMap, BTreeSet};
use tracing::debug;

pub struct TaskNameShortener {
    names: Vec<String>,
    separator: String,
    substrings: BTreeMap<(usize, String), BTreeSet<usize>>,
}

impl TaskNameShortener {
    fn init_substrings(
        names: &Vec<String>,
        separator: &String
    ) -> BTreeMap<(usize, String), BTreeSet<usize>> {
        let mut substrings = BTreeMap::new();
        for (i, name) in names.iter().enumerate() {
            debug!("Adding name: ({}, {})", i, name);
            for (j, split) in name.split(separator).enumerate() {
                debug!("Adding split: ({}, {})", j, split);
                (
                    *substrings
                    .entry((j, split.to_string()))
                    .or_insert(BTreeSet::new())
                ).insert(i);
            }
        }
        substrings
    }
    pub fn new(names: Vec<String>, separator: String) -> Self {
        let substrings = Self::init_substrings(&names, &separator);
        Self{ names, separator, substrings}
    }
    fn try_shorten(&mut self, substring: &str) -> bool {
        let mut shorter_names = LinkedHashSet::new();
        for elem in self.names.iter() {
            let shortened = elem.replace(substring, "");
            if shortened.len() == 0 {
                return false;
            }
            shorter_names.insert(shortened);
        }
        debug!("{:?}", shorter_names);
        if shorter_names.len() == self.names.len() {
            self.names = shorter_names.into_iter().collect();
            self.substrings = Self::init_substrings(&self.names, &self.separator);
            return true;
        }
        return false;
    }
    pub fn run(mut self) -> Vec<String> {
        self.substrings = Self::init_substrings(&self.names, &self.separator);
        if self.names.len() == 1 {
            debug!("Short-circuited task shortening.");
            return vec![self.names.get(0).unwrap().split("_").next().unwrap().to_string()];
        }
        loop {
             let max_val = self.substrings.iter().filter(
                |(k, v)| v.len() > 1 || k.0 > 0
             ).max_by_key(
                |((_i, k), v)| k.len() * v.len()
             ).map(|((i, k), v)| (i.clone(), k.clone(), v.clone()));
             if let Some((i, max_key, _)) = max_val {
                if max_key.len() == 0 {
                    break;
                }
                let key = match i {
                    0 => format!("{}{}", &max_key, self.separator),
                    _ => format!("{}{}", self.separator, &max_key),
                };
                debug!("Trying to shorten with max_key = {}", key);
                if !&self.try_shorten(&key) {
                    break;
                }
             } else {
                debug!("Could not find max_key.");
                break;
             }
        }
        self.names
    }
}
