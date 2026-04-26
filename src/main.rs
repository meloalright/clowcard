mod art;
mod card;
mod display;

use card::{CardData, CARDS_YAML};
use display::{print_card, print_card_both, print_card_json};
use rand::seq::SliceRandom;
use std::env;

fn parse_lang(args: &[String]) -> &str {
    for a in args {
        match a.as_str() {
            "--zh" => return "zh",
            "--ja" => return "ja",
            "--en" => return "en",
            _ => {}
        }
    }
    "en"
}

fn has_flag(args: &[String], flag: &str) -> bool {
    args.iter().any(|a| a == flag)
}

fn main() {
    let data: CardData = serde_yaml::from_str(CARDS_YAML).expect("failed to parse cards.yaml");
    let args: Vec<String> = env::args().collect();
    let lang = parse_lang(&args);

    if args.len() > 1 {
        match args[1].as_str() {
            "-i" | "--id" => {
                if let Some(id) = args.get(2) {
                    if let Some(card) = data.cards.iter().find(|c| c.id == *id) {
                        if has_flag(&args, "--json") {
                            let reversed = rand::random::<bool>();
                            print_card_json(card, reversed, lang);
                        } else {
                            print_card_both(card, lang);
                        }
                        return;
                    }
                }
                eprintln!("not found, use -h to see help");
                std::process::exit(1);
            }
            "-V" | "--version" => {
                println!("clowcard {}", env!("CARGO_PKG_VERSION"));
                return;
            }
            "-h" | "--help" => {
                println!("clowcard - clow card divination");
                println!();
                println!("  clowcard            random draw");
                println!("  clowcard --en       random draw (english)");
                println!("  clowcard --zh       random draw (chinese)");
                println!("  clowcard --ja       random draw (japanese)");
                println!("  clowcard --json     output as JSON");
                println!("  clowcard -i <id>    by id (shows both positions)");
                println!("  clowcard -V         version");
                println!("  clowcard -h         help");
                return;
            }
            "--en" | "--zh" | "--ja" | "--json" => {}
            _ => {
                eprintln!("unknown: {}, use -h", args[1]);
                std::process::exit(1);
            }
        }
    }

    let mut rng = rand::thread_rng();
    let card = data.cards.choose(&mut rng).unwrap();
    let reversed = rand::random::<bool>();
    if has_flag(&args, "--json") {
        print_card_json(card, reversed, lang);
    } else {
        print_card(card, reversed, lang);
    }
}
