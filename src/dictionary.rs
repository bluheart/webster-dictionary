use crate::Row;
use csv::ReaderBuilder;
use std::collections::HashMap;
use crate::trie::Trie;
use crate::draw;

pub fn define(word: String, alphabet: &HashMap<char, usize>, data: memmap::Mmap) {
    let mut start: usize = 0;
    let mut end: usize = 0;
    match word.chars().next() {
        Some(first) => {
            match alphabet.get(&first) {
                Some(dig) => {
                    start = *dig;
                }
                None => {}
            }
            match alphabet.get(&(((first as u8) + 1) as char)) {
                Some(dig) => {
                    end = *dig;
                }
                None => {}
            }
        }
        None => {}
    }
    let mut trie = Trie::new();
    let mut rdr = ReaderBuilder::new()
        .delimiter(b'@')
        .from_reader(&data[start..end]);
    for result in rdr.records() {
        let record = result.unwrap();
        let row: Row = record.deserialize(None).unwrap();
        trie.new_word(row);
    }
    let found = trie.search(word.to_string());
    draw::print_entry(found);
}
