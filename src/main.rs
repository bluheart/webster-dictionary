extern crate termion;
mod trie;
mod address;
use std::io::Read;
use address::*;
use std::error::Error;
use trie::Trie;
use csv::ReaderBuilder;
use std::fs::OpenOptions;
use std::collections::HashMap;
use std::io;
use std::char::from_u32;
use termion::{color, style};

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
    let mut alphabet = HashMap::new();
    alphabet.insert('A', A);
    alphabet.insert('B', B);
    alphabet.insert('C', C);
    alphabet.insert('D', D);
    alphabet.insert('E', E);
    alphabet.insert('F', F);
    alphabet.insert('G', G);
    alphabet.insert('H', H);
    alphabet.insert('I', I);
    alphabet.insert('J', J);
    alphabet.insert('K', K);
    alphabet.insert('L', L);
    alphabet.insert('M', M);
    alphabet.insert('N', N);
    alphabet.insert('O', O);
    alphabet.insert('P', P);
    alphabet.insert('Q', Q);
    alphabet.insert('R', R);
    alphabet.insert('S', S);
    alphabet.insert('T', T);
    alphabet.insert('U', U);
    alphabet.insert('V', V);
    alphabet.insert('W', W);
    alphabet.insert('X', X);
    alphabet.insert('Y', Y);
    alphabet.insert('Z', Z);
    alphabet.insert('[', END);
    let dict = OpenOptions::new()
        .read(true)
        .open("webster.csv")
        .expect("Unable to open file");

    let data = unsafe {
        memmap::MmapOptions::new()
            .map(&dict)
            .expect("Could not access data from memory mapped file.")
    };

    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        buffer = buffer.trim().to_string();
        let start: usize;
        let end: usize;
        match buffer.chars().next() {
            Some(first) => {
                match alphabet.get(&first) {
                    Some(dig) =>  {start = *dig;}
                    None => {
                        println!("Invalid word!");
                        continue;
                    }
                }
                match alphabet.get(&(((first as u8)+1) as char)) {
                    Some(dig) =>  {end = *dig;}
                    None => {
                        println!("Invalid word!");
                        continue;
                    }
                }
            }
            None => {
                println!("Invalid word!");
                continue;
            }
        }
        let mut trie = Trie::new();
        let mut rdr = ReaderBuilder::new().delimiter(b'@').from_reader(&data[start..end]);
        for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
            let record = result?;
            let row: Row = record.deserialize(None)?;
            trie.new_word(row);
            //println!("{:?}", row);
        }
        let found = trie.search(buffer);
        let mut res: String;
        match found {
            Some(entry) => {
                res = format!("\n{}{}[{}]{}{}", style::Bold, color::Fg(color::Cyan), entry.word, color::Fg(color::Reset),style::Reset);
                for def in entry.definitions {
                    res = format!("{}\n{bold}\n{def}{reset}", res, bold=style::Bold , def=def, reset=style::Reset );
                }
            }
            None => {
                res = format!("{}{}Not found!{}{}", style::Bold, color::Fg(color::Red), color::Fg(color::Reset), style::Reset);
            }
        }
        println!("{}\n\n",res);
    }
    Ok(())
}
