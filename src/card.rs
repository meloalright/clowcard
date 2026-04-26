use serde::Deserialize;

#[derive(Deserialize)]
pub struct CardData {
    pub cards: Vec<Card>,
}

#[derive(Deserialize)]
pub struct Card {
    pub id: String,
    pub zh: String,
    pub en: String,
    pub fortune: Fortune,
}

#[derive(Deserialize)]
pub struct Fortune {
    pub upright: String,
    pub reversed: String,
    pub upright_zh: String,
    pub reversed_zh: String,
    pub upright_ja: String,
    pub reversed_ja: String,
}

pub const CARDS_YAML: &str = include_str!("../data/cards.yaml");
