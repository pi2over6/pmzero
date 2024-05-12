use super::{db, members};
pub use db::Game;

use chrono::NaiveDate;
use std::collections::HashMap;
use std::error::Error;

pub fn games_filtered(game_filter: HashMap<String, String>) -> Result<Vec<Game>, Box<dyn Error>> {
    let mut games = db::load_games()?;

    if game_filter.contains_key("ranking") && game_filter["ranking"] == "true" {
        games.retain(|game| !game.non_rank_game);
    }

    if game_filter.contains_key("date") && game_filter["date"] == "on" {
        let start = NaiveDate::parse_from_str(&game_filter["start"], "%Y-%m-%d")?;
        let end = NaiveDate::parse_from_str(&game_filter["end"], "%Y-%m-%d")?;

        games.retain(|game| game.in_date(&start, &end));
    }

    if game_filter.contains_key("ind") && game_filter["ind"] == "on" {
        let member_id = members::get_member_id(&game_filter["this_member"])?;
        games.retain(|game| game.member_in_game(&member_id));
    }

    if game_filter.contains_key("vs") && game_filter["vs"] == "on" {
        let opponent_id = members::get_member_id(&game_filter["opponent"])?;
        games.retain(|game| game.member_in_game(&opponent_id));
    }

    Ok(games)
}

pub fn append_game(game: &Game) -> Result<(), Box<dyn Error>> {
    let mut games = db::load_games()?;

    let mut game_cloned = game.clone();
    game_cloned.id = games.len();

    games.push(game_cloned);

    db::save_games(&games)?;
    Ok(())
}

impl db::Game {
    fn in_date(&self, start: &NaiveDate, end: &NaiveDate) -> bool {
        let (game_time, _) =
            &NaiveDate::parse_and_remainder(&self.recorded_at, "%Y-%m-%d").unwrap();
        start < game_time && game_time < end
    }

    fn member_in_game(&self, member_id: &usize) -> bool {
        self.scores
            .into_iter()
            .find(|x| &x.0 == member_id)
            .is_some()
    }

    pub fn calculate_points(&self) -> [(f32, u8); 4] {
        let mut results: Vec<(i32, u8)> = self.scores.into_iter().map(|x| x.1).zip(0..).collect();
        results.sort();
        results.reverse();

        let mut points = [(0.0, 0); 4];
        points[0].0 = results[0].0 as f32 / 1000.0 + 10.0;
        points[1].0 = results[1].0 as f32 / 1000.0 - 20.0;
        points[2].0 = results[2].0 as f32 / 1000.0 - 40.0;
        points[3].0 = results[3].0 as f32 / 1000.0 - 50.0;

        for ii in 0..4 {
            points[ii].1 = results[ii].1;
        }

        points
    }

    pub fn get_only_scores(&self) -> [i32; 4] {
        [
            self.scores[0].1,
            self.scores[1].1,
            self.scores[2].1,
            self.scores[3].1,
        ]
    }
}
