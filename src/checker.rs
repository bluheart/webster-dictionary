use crate::trie::Trie;
use crate::Row;
use csv::ReaderBuilder;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::iter::FromIterator;
use termion::{color, style};
use indicatif::{ProgressBar, ProgressStyle};

pub fn check(mut file: File, alphabet: &HashMap<char, usize>, data: memmap::Mmap) {
    let lowercase = "abcdefghijklmnopqrstuvwxyz";

    let word_re = Regex::new(r"[\w']{2,}").unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    text = Regex::new(r"’").unwrap().replace_all(&text, "'").to_string();
    let mut words: Vec<String>;
    {
        words = word_re
            .find_iter(&text)
            .map(|w| w.as_str().to_string())
            .collect();
        let mut word_set: HashSet<String> = HashSet::from_iter(words.iter().cloned());
        words = word_set.drain().collect();
    }
    let bar = ProgressBar::new(alphabet.len() as u64 -3);
    bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{bar:40.cyan/blue}] ({eta})")
        .progress_chars("#>-"));
    let mut trie = Trie::new();
    for l in lowercase.chars() {
        bar.inc(1);
        let mut start: usize = 0;
        let mut end: usize = 0;
        match alphabet.get(&l) {
            Some(dig) => {
                start = *dig;
            }
            None => {}
        }
        match alphabet.get(&(((l as u8) + 1) as char)) {
            Some(dig) => {
                end = *dig;
            }
            None => {}
        }
        let mut rdr = ReaderBuilder::new()
            .delimiter(b'@')
            .from_reader(&data[start..end]);
        for result in rdr.records() {
            let record = result.unwrap();
            let row: Row = record.deserialize(None).unwrap();
            trie.new_word(row);
        }
        for original in &words {
            let w = original.to_lowercase();
            let first = w.chars().next().unwrap();
            if first == l {
                // check if word is in trie
                if !trie.check(w.to_string()) {
                    //what if the word is a plural?
                    if w.chars().last().unwrap() == 's' {
                        let mut word  = &w[..w.len()-1];
                        if trie.check(word.to_string()) { continue;}
                        word = &w[..w.len()-2];
                        if trie.check(word.to_string()) { continue;}
                    }
                    //not in trie
                    let repl_re = Regex::new(original).unwrap();
                    let word = format!( "{bold}{red}{word}{reset_c}{reset_s}",
                        bold = style::Bold,
                        red = color::Fg(color::Red),
                        word = original,
                        reset_c = color::Fg(color::Reset),
                        reset_s = style::Reset,
                    );
                    text = repl_re.replace_all(&text, word.as_str()).to_string();
                }
            } else {
                continue;
            }
        }
    }
    println!("\n\n{}\n",text);
}
