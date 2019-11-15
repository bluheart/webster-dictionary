extern crate termion;
use std::env;
mod address;
mod dictionary;
mod draw;
mod trie;
use std::error::Error;
use std::fs::OpenOptions;


#[macro_use]
extern crate serde_derive;
extern crate memmap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Row {
    word: String,
    definition: String,
}

#[derive(Debug)]
pub struct Entry {
    word: String,
    definitions: Vec<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let alphabet = address::new();
    //open file
    let dict = OpenOptions::new()
        .read(true)
        .open("webster.csv")
        .expect("Unable to open file");

    //create memory map
    let data = unsafe {
        memmap::MmapOptions::new()
            .map(&dict)
            .expect("Could not access data from memory mapped file.")
    };

    if &args[1] == "define" {
        dictionary::define(args[2].trim().to_lowercase().to_string(), &alphabet, data);
    }
    Ok(())
}
