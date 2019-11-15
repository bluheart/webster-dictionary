use crate::Entry;
use crate::Row;
use std::collections::HashMap;
type Alpha = Option<Box<HashMap<char, Node>>>;

pub struct Node {
    terminal: bool,
    alpha: Alpha,
    definitions: Option<Vec<String>>,
}

pub struct Trie {
    root: Node,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: Node {
                terminal: false,
                alpha: Some(Box::new(HashMap::new())),
                definitions: None,
            },
        }
    }

    //loads a word into Trie
    pub fn new_word(&mut self, row: Row) {
        let mut iter = &mut self.root;
        for c in row.word.chars() {
            if let Some(ref mut alpha) = iter.alpha {
                if alpha.contains_key(&c) {
                    iter = alpha.get_mut(&c).unwrap();
                } else {
                    alpha.insert(
                        c,
                        Node {
                            terminal: false,
                            alpha: Some(Box::new(HashMap::new())),
                            definitions: None,
                        },
                    );
                    iter = alpha.get_mut(&c).unwrap();
                }
            }
        }
        iter.terminal = true;
        if let Some(ref mut definition) = iter.definitions {
            definition.push(row.definition);
        } else {
            iter.definitions = Some(vec![row.definition]);
        }
    }

    //search Trie for word
    pub fn search(&mut self, word: String) -> Option<Entry> {
        let mut iter: &mut Node = &mut self.root;
        for c in word.chars() {
            if let Some(ref mut alpha) = iter.alpha {
                if alpha.contains_key(&c) {
                    iter = alpha.get_mut(&c).expect("couldn't get node");
                } else {
                    return None;
                }
            }
        }
        if iter.terminal {
            Some(Entry {
                word: word,
                definitions: iter.definitions.as_ref().unwrap().to_vec(),
            })
        } else {
            None
        }
    }
}
