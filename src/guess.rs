use std::{collections::HashMap, path::MAIN_SEPARATOR};

use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Scoring {
    pub in_word: bool,
    pub correct_idx: bool,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CharacterInfo {
    pub char: String,
    pub scoring: Scoring,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub guess: String,
    pub was_correct: Option<bool>,
    pub character_info: Option<Vec<CharacterInfo>>,
}

pub fn guess(client: &Client, word: &str) -> anyhow::Result<Response> {
    let mut map = HashMap::new();
    map.insert("guess", word);
    let res = client
        .post("https://wordle-api.vercel.app/api/wordle")
        .json(&map)
        .send()?
        .json::<Response>()?;


    Ok(res)
}
