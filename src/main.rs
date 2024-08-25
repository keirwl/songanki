use clap::Parser;
use furi::get_reading_dict;
use reqwest::blocking::Client;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{self, BufRead, Write};
use std::fs;

mod furi;
use crate::furi::get_dict;


const URL: &str = "http://localhost:8765/";
const MARU_ONE: u32 = 0x2460;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    deck_name: String,
    #[arg(short, long)]
    model_name: String,
    file_path: String,
}

// let result = parse(&dict, "これを持っていけ").unwrap();
// for token in &result.0
// {
//     println!("{}", token.feature);
// }
// let split_up_string = tokenstream_to_string(&result.0, "|");
// println!("{}", split_up_string);
// assert_eq!(split_up_string, "これ|を|持っ|て|いけ");

fn get_pairs_from_file(file_path: &str) -> Vec<(String, String)> {
    print!("reading lyrics file {}...", file_path);
    io::stdout().flush().unwrap();
    let contents: Vec<String> = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect();

    let mut seen_pairs: HashSet<(&String, &String)> = HashSet::new();
    let mut count: HashMap<&String, u32> = HashMap::new();
    let mut pairs: Vec<(String, String)> = Vec::with_capacity(contents.len());

    for i in 0..=contents.len() - 2 {
        let pair = (&contents[i], &contents[i + 1]);
        if seen_pairs.contains(&pair) {
            continue;
        }
        seen_pairs.insert(pair);

        let card_back = pair.1.clone();
        let card_front;
        let n = count.entry(&pair.0).and_modify(|n| *n += 1).or_insert(1);
        if *n > 1 {
            card_front = format!("{}　{}", char::from_u32(MARU_ONE + *n - 1).unwrap(), pair.0);
        } else {
            card_front = pair.0.clone();
        }
        pairs.push((card_front, card_back));
    }
    println!(" done");
    pairs
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let pairs = get_pairs_from_file(&args.file_path);
    let _deck_name = args.deck_name;
    let _model_name = args.model_name;

    let dict = get_dict();
    for (card_front, _) in &pairs[0..3] {
        println!("{}:", card_front);
        let (tokens, _) = dict.tokenize(card_front).unwrap();
        for token in tokens {
            println!("{}", token.get_feature(&dict));
        }
    }

    let reading_dict = get_reading_dict();
    // for (key, val) in reading_dict.iter() {
    //     println!("{}: {:?}", key, val);
    // }

    // let mut notes: Vec<String> = Vec::new();
    // let mut first = true;
    // let mut tag: String = "".to_string();
    // for (card_front, card_back) in pairs {
    //     if first {
    //         tag = card_front.clone();
    //         first = false;
    //     }
    //     notes.push(format!(
    //         r#"{{
    //             "deckName": "{deck_name}",
    //             "modelName": "{model_name}",
    //             "tags": ["{tag}"],
    //             "fields": {{
    //                 "This line": "{card_front}",
    //                 "This line reading": "{card_front}",
    //                 "Next line": "{card_back}",
    //                 "Next line reading": "{card_back}"
    //             }}
    //         }}"#
    //     ));
    // }

    // let payload = format!(
    //     r#"
    // {{
    //   "action": "addNotes",
    //   "version": 6,
    //   "params": {{
    //     "notes": [{notes_list}]
    //   }}
    // }}"#,
    //     notes_list = notes.join(",")
    // );

    // let client = Client::new();
    // let resp = client.post(URL).body(payload).send()?;
    // println!("{}", resp.status());
    // println!("{}", resp.text()?);
    Ok(())
}
