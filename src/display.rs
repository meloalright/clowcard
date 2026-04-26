use crate::art::get_sharpen_stroke_art;
use crate::card::Card;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn flush() {
    io::stdout().flush().unwrap();
}

pub fn print_card(card: &Card, reversed: bool, lang: &str) {
    println!();
    println!("{} ({})", card.zh, card.en);
    flush();
    thread::sleep(Duration::from_secs(1));
    match lang {
        "zh" => {
            let f = if reversed { &card.fortune.reversed_zh } else { &card.fortune.upright_zh };
            println!("{}", f);
        }
        "ja" => {
            let f = if reversed { &card.fortune.reversed_ja } else { &card.fortune.upright_ja };
            println!("{}", f);
        }
        _ => {
            let f = if reversed { &card.fortune.reversed } else { &card.fortune.upright };
            println!("{}", f);
        }
    }
    flush();
    thread::sleep(Duration::from_secs(1));
    let art = get_sharpen_stroke_art(&card.id);
    if !art.is_empty() {
        println!();
        for line in art.lines() {
            println!("{}", line);
            flush();
            thread::sleep(Duration::from_millis(100));
        }
    }
}

pub fn print_card_json(card: &Card, reversed: bool, lang: &str) {
    let (upright, reversed_text) = match lang {
        "zh" => (&card.fortune.upright_zh, &card.fortune.reversed_zh),
        "ja" => (&card.fortune.upright_ja, &card.fortune.reversed_ja),
        _ => (&card.fortune.upright, &card.fortune.reversed),
    };
    let position = if reversed { "reversed" } else { "upright" };
    let fortune = if reversed { reversed_text } else { upright };
    println!(
        r#"{{"id":"{}","zh":"{}","en":"{}","position":"{}","upright":"{}","reversed":"{}","fortune":"{}"}}"#,
        card.id, card.zh, card.en, position, upright, reversed_text, fortune
    );
}

pub fn print_card_both(card: &Card, lang: &str) {
    println!();
    println!("{} ({})", card.zh, card.en);
    println!();
    match lang {
        "zh" => {
            println!("{}", card.fortune.upright_zh);
            println!("{}", card.fortune.reversed_zh);
        }
        "ja" => {
            println!("{}", card.fortune.upright_ja);
            println!("{}", card.fortune.reversed_ja);
        }
        _ => {
            println!("{}", card.fortune.upright);
            println!("{}", card.fortune.reversed);
        }
    }
    let art = get_sharpen_stroke_art(&card.id);
    if !art.is_empty() {
        println!();
        print!("{}", art);
    }
}
