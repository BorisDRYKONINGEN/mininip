//! An higher-level representation API for the data returned by parsers
//! 
//! # See
//! `Tree` to convert a `HashMap<Identifier, Value>` into a more user-friendly data-type
//! 
//! `Section` to list the keys inside a section

use crate::datas::{Identifier, Value};
use std::collections::{HashMap, hash_map};

/// A more user-friendly data-type to represent the data returned by `parser::Parser::data`
/// 
/// # Example
/// ```
/// use mininip::datas{Identifier, Value, self};
/// use datas::tree::Tree;
/// use mininip::parse::parse_file;
/// 
/// let tree = Tree::from_data(parse_file("good.ini").unwrap());
/// for i in tree.sections() {
///     println!("[{}] ; Section {}", i, i);
///     for j in i.keys() {
///         println!("{}={} ; key {}", j.ident().name(), j.value(), j.ident().name());
///     }
/// }
/// ```
pub struct Tree {
    cache: Cache,
    data: HashMap<Identifier, Value>,
}

impl Tree {
    /// Creates a `Tree` from the data returned by a parser
    pub fn from_data(data: HashMap<Identifier, Value>) -> Tree {
        Tree {
            cache: Cache::from(&data),
            data: data,
        }
    }
}


/// A cached result of an extraction of all the section and keys names. Will be
/// kept and updated forever in the owning `Tree`
struct Cache {
    /// An ordered list of sections
    sections: Vec<String>,
    /// A map associating a section name to an ordered list of key names
    keys: HashMap<Option<String>, Vec<String>>,
}

impl From<&HashMap<Identifier, Value>> for Cache {
    fn from(data: &HashMap<Identifier, Value>) -> Cache {
        let mut sections = Vec::new();
        let mut keys = HashMap::<_, Vec<String>>::new();

        for i in data.keys() {
            let section_name = match i.section() {
                Some(val) => Some(String::from(val)),
                None      => None,
            };

            match keys.entry(section_name.clone()) {
                hash_map::Entry::Occupied(mut entry) => entry.get_mut().push(String::from(i.name())),
                hash_map::Entry::Vacant(entry)       => {
                    let vec = vec![String::from(i.name())];
                    entry.insert(vec);

                    if let Some(val) = section_name {
                        sections.push(val);
                    }
                },
            }
        }

        // No collisions so unstable sorting is more efficient
        sections.sort_unstable();

        if let Some(val) = keys.get_mut(&None) {
            val.sort_unstable();
        }
        for i in &sections {
            keys.get_mut(&Some(i.clone()))
                .expect("Any section name in `section` should be in `keys`")
                .sort_unstable();
        }

        Cache {
            sections,
            keys,
        }
    }
}


#[cfg(test)]
mod tests;
