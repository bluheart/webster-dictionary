mod trie;
mod address;
use address::*;
use std::error::Error;
use trie::Trie;
use csv::ReaderBuilder;
use std::fs::OpenOptions;

#[macro_use]
extern crate serde_derive;
extern crate memmap;

#[derive(Serialize, Deserialize,Debug)]
pub struct Row {
    word: String,
    definition: String
}

#[derive(Debug)]
pub struct Entry {
    word: String,
    definitions: Vec<String>
}

fn main() -> Result<(), Box<dyn Error>> {

    let mut trie = Trie::new();

    let dict = OpenOptions::new()
        .read(true)
        .open("webster.csv")
        .expect("Unable to open file");

    let data = unsafe {
        memmap::MmapOptions::new()
            .map(&dict)
            .expect("Could not access data from memory mapped file.")
    };

    let mut rdr = ReaderBuilder::new().delimiter(b'@').from_reader(&data[Z..]);
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        let row: Row = record.deserialize(None)?;
        trie.new_word(row);
        //println!("{:?}", row);
    }
    println!("{:?}", trie.search("Zeta".to_string()));
    Ok(())
}
