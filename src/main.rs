use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::process;

const URL: &str = "http://localhost:8765/";
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

    let mut pairs: Vec<(String, String)> = Vec::with_capacity(contents.len() - 1);
    for i in 0..=contents.len() - 2 {
        pairs.push((contents[i].clone(), contents[i + 1].clone()));
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

    let mut cards: HashMap<String, Vec<String>> = HashMap::new();
    for (former, latter) in pairs.into_iter() {
        let pair = cards.entry(former).or_insert(Vec::new());
        if !pair.contains(&latter) {
            pair.push(latter);
        }
    }

    for (key, value) in cards {
        for (i, latter) in value.iter().enumerate() {
            let mut card_front = key.clone();
            if i > 1 {
                card_front = format!("{}: {}", i, card_front);
            }
            let card_back = latter;
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
}
