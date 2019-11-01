extern crate memmap;
use std::error::Error;
#[macro_use]
extern crate serde_derive;
use regex::Regex;
use std::{
    fs::OpenOptions,
    io::{Seek, SeekFrom, Write},
};

const SIZE: u64 = 1024 * 1024;

#[derive(Deserialize)]
struct Row {
    entry: String
}

#[derive(Serialize)]
struct Entry {
    word: String,
    definition: String
}

fn main() -> Result<(), Box<dyn Error>> {

    let test = "Azured, Of an azure color; sky-blue.";
    let mut f = OpenOptions::new()
        .read(true)
        .open("A.csv")
        .expect("Unable to open file");
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("oxford.csv")
        .expect("Unable to open file");

    file.seek(SeekFrom::Start(978000)).unwrap();
    file.write_all(&[0]).unwrap();
    file.seek(SeekFrom::Start(0)).unwrap();

    let data = unsafe {
        memmap::MmapOptions::new()
            .map(&f)
            .expect("Could not access data from memory mapped file")
    };

    let mut output = unsafe {
        memmap::MmapOptions::new()
            .map_mut(&file)
            .expect("Could not access data from memory mapped file")
    };
    println!("{:?}", data);

    let mut rdr = csv::Reader::from_reader(&data[..]); //0-400055, 400055-..
    let re = Regex::new(r"([\w -]{1,30}) \(.*\) ?(.*)?").unwrap();
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        let row: Row = record.deserialize(None)?;
        let find = re.captures(&row.entry).unwrap();
        println!("{:?}", find.get(1).map_or("", |m| m.as_str()));
    }
    Ok(())
}
