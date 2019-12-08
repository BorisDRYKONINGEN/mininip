//! Provides a `Dumper` structure which creates a new INI file content

use crate::datas::{Identifier, Value};
use std::collections::{hash_map, HashMap};

/// A stated object, which from couples of `Identifier` and `Value`, creates a new INI tree, directly dumpable into a new file
/// Each entry of the `tree` member has for key the section name and for value a list of lines
#[derive(Debug)]
pub struct Dumper {
    tree: HashMap<Option<String>, Vec<String>>,
}

impl Dumper {
    /// Creates a new `Dumper` object
    pub fn new() -> Dumper {
        Dumper {
            tree: HashMap::new(),
        }
    }

    /// Dumps a couple `Identifier` / `Value` into the `Dumper`
    pub fn dump(&mut self, identifier: Identifier, value: Value) {
        let line = format!("{}={}", identifier.name(), value.dump());

        let key = match identifier.section() {
            Some(val) => Some(String::from(val)),
            None      => None,
        };
        match self.tree.entry(key) {
            hash_map::Entry::Occupied(mut entry) => entry.get_mut().push(line),
            hash_map::Entry::Vacant(entry)       => { entry.insert(vec![line]); },
        }
    }

    /// Generates a `String` containing the code of the INI data stored in the `Dumper`
    pub fn generate(mut self) -> String {
        // We want the sections to be sorted by name
        let mut sections: Vec<String> = Vec::with_capacity(self.tree.len());
        for (key, _value) in self.tree.iter() {
            if let Some(val) = key {
                sections.push(val.clone());
            }
        }
        sections.sort();

        // And None to be the first one
        let mut result = String::new();
        if let Some(val) = self.tree.get_mut(&None) {
            val.sort();
            for i in val {
                result.push_str(i);
                result.push('\n');
            }

            result.push('\n');
        }

        for i in sections {
            result.push('[');
            result.push_str(&i);
            result.push_str("]\n");

            for j in &self.tree[&Some(i)] {
                result.push_str(j);
                result.push('\n');
            }

            result.push('\n');
        }

        result.pop();
        result
    }
}


#[cfg(test)]
mod tests;
