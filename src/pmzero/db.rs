use std::{collections::HashMap, error::Error};

pub type Members = HashMap<usize, String>;

pub fn load_games() -> Result<Vec<Game>, Box<dyn Error>> {
    let mut games: Vec<Game> = serde_json::from_str(&std::fs::read_to_string("games.json")?)?;
    games.reverse();
    Ok(games)
}

pub fn load_members() -> Result<Members, Box<dyn Error>> {
    let members = serde_json::from_str(&std::fs::read_to_string("members.json")?)?;
    Ok(members)
}

pub fn save_games(games: &Vec<Game>) -> Result<(), std::io::Error> {
    std::fs::write("games.json", serde_json::to_string(games)?)
}

pub fn save_members(members: &Members) -> Result<(), std::io::Error> {
    std::fs::write("members.json", serde_json::to_string(members)?)
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Game {
    pub id: usize,
    pub non_rank_game: bool,
    pub used_dora_count: Option<u8>,
    pub recorded_at: String,
    pub scores: [(usize, i32); 4],
    pub leftover_score: i32,
    pub remarks: String,
}
