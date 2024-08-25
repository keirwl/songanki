use std::fs::File;
use std::io::{BufRead, Write};
use std::{collections::HashMap, io};

use notmecab::{Blob, Cache, Dict};

pub const HIRA_START: char = '\u{3041}';
pub const HIRA_END: char = '\u{309F}';
pub const KATA_START: char = '\u{30A1}';
pub const KATA_END: char = '\u{30FF}';
pub const KATA_SHIFTABLE_START: char = '\u{30A1}';
pub const KATA_SHIFTABLE_END: char = '\u{30F6}';

pub fn kata_to_hira(c: char) -> char {
    if KATA_SHIFTABLE_START <= c && c <= KATA_SHIFTABLE_END {
        let z = c as u32 + HIRA_START as u32 - KATA_START as u32;
        char::from_u32(z).expect(&format!("impossible: not katakana: {}", c))
    } else {
        c
    }
}

pub fn get_reading_dict() -> HashMap<String, Vec<String>> {
    print!("reading dictionary for readings...");
    io::stdout().flush().unwrap();
    let mut dict: HashMap<String, Vec<String>> = HashMap::new();
    let file = File::open("data/pitch_accents_formatted.csv").unwrap();
    for line in std::io::BufReader::new(file).lines() {
        let parts = line.unwrap().splitn(3, '\t')
                                 .map(|p| p.to_string())
                                 .collect::<Vec<_>>();
        let word = parts[0].clone();
        let reading = parts[1].chars().map(kata_to_hira).collect::<String>();
        dict.entry(word)
            .and_modify(|r| r.push(reading.clone()))
            .or_insert(vec![reading]);
    }
    println!(" done");
    dict
}

pub fn get_dict() -> Dict {
    print!("reading dictionaries for mecab...");
    io::stdout().flush().unwrap();
    let sysdic = Blob::open("data/sys.dic").unwrap();
    let unkdic = Blob::open("data/unk.dic").unwrap();
    let matrix = Blob::open("data/matrix.bin").unwrap();
    let unkdef = Blob::open("data/char.bin").unwrap();

    let dict = Dict::load(sysdic, unkdic, matrix, unkdef).unwrap();
    println!(" done");
    dict
}

// pub fn annotate_line(line: &str, dict: &Dict, cache: &mut Cache) -> str {
//     let mut tokens = Vec::new();
//     dict.tokenize_with_cache(cache, line, &mut tokens).unwrap();
// }
