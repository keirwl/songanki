use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::process;

const URL: &str = "http://localhost:8765/";
const MARU_ONE: u32 = 0x2460;
const ADD_NOTE: &str = r#"
{
  "action": "addNotes",
  "version": 6,
  "params": {
    "notes": [
      {
        "deckName": "Test",
        "modelName": "Song format",
        "fields": {
          "This line": "card_front",
          "Next line reading": "card_back"
        }
      }
    ]
  }
}"#;

fn get_pairs_from_file(file_path: &str) -> Vec<(String, String)> {
    println!("Reading file {}", file_path);
    let contents: Vec<String> = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect();

    let mut seen_pairs: HashSet<(&String, &String)> = HashSet::new();
    let mut count: HashMap<&String, u32> = HashMap::new();
    let mut pairs: Vec<(String, String)> = Vec::with_capacity(contents.len() - 1);

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
    pairs
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Takes exactly one file path argument");
        process::exit(1);
    }
    let file_path = &args[1];
    let pairs = get_pairs_from_file(&file_path);

    for (card_front, card_back) in pairs {
        let body = format!(
            r#"
		{{
  	  	  "action": "addNotes",
  	  	  "version": 6,
  	  	  "params": {{
    		"notes": [
      	  	  {{
        		"deckName": "Test",
        		"modelName": "Song format",
        		"fields": {{
          	  	  "This line": "{card_front}",
          	  	  "Next line reading": "{card_back}"
        		}}
      	  	  }}
    		]
  	  	  }}
		}}"#,
            card_front = card_front,
            card_back = card_back
        );

        // let client = reqwest::blocking::Client::new();
        // let res = client
        //     .post(URL)
        //     .body(body)
        //     .send()
        //     .unwrap();
        println!("{}, {}", card_front, card_back);
    }
}
