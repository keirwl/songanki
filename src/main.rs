use clap::Parser;
use reqwest::blocking::Client;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

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

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let pairs = get_pairs_from_file(&args.file_path);
    let deck_name = args.deck_name;
    let model_name = args.model_name;

    let mut notes: Vec<String> = Vec::new();
    let mut first = true;
    let mut tag: String = "".to_string();
    for (card_front, card_back) in pairs {
        if first {
            tag = card_front.clone();
            first = false;
        }
        notes.push(format!(
            r#"{{
                "deckName": "{deck_name}",
                "modelName": "{model_name}",
                "tags": ["{tag}"],
                "fields": {{
                    "This line": "{card_front}",
                    "This line reading": "{card_front}",
                    "Next line": "{card_back}",
                    "Next line reading": "{card_back}"
                }}
            }}"#
        ));
    }

    let payload = format!(
        r#"
    {{
      "action": "addNotes",
      "version": 6,
      "params": {{
        "notes": [{notes_list}]
      }}
    }}"#,
        notes_list = notes.join(",")
    );

    let client = Client::new();
    let resp = client.post(URL).body(payload).send()?;
    println!("{}", resp.status());
    println!("{}", resp.text()?);
    Ok(())
}
