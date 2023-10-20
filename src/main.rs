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
    let deck_name: String = "".to_string();
    let model_name: String = "".to_string();

    let mut notes: Vec<String> = Vec::new();
    for (card_front, card_back) in pairs {
        notes.push(format!(
            r#"{{
        		"deckName": "{deck_name}",
        		"modelName": "{model_name}",
        		"fields": {{
          	  	  "T"this line": "{card_front}",
          	  	  "T"this line reading": "{card_front}",
          	  	  "N"next line": "{card_back}"
          	  	  "N"next line reading": "{card_back}"
        		}}
      	  	}}"#,
            card_front = card_front,
            card_back = card_back
        ));
        println!("{}, {}", card_front, card_back);
    }
}
