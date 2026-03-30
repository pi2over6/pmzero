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

    let start_val = game_filter.get("start").map(|s| s.as_str()).unwrap_or("");
    let end_val = game_filter.get("end").map(|s| s.as_str()).unwrap_or("");
    if !start_val.is_empty() || !end_val.is_empty() {
        let start = if start_val.is_empty() {
            NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()
        } else {
            NaiveDate::parse_from_str(start_val, "%Y-%m-%d")?
        };
        let end = if end_val.is_empty() {
            NaiveDate::from_ymd_opt(9999, 12, 31).unwrap()
        } else {
            NaiveDate::parse_from_str(end_val, "%Y-%m-%d")?
        };
        games.retain(|game| game.in_date(&start, &end));
    }

    let ind_val = game_filter.get("ind").map(|s| s.as_str()).unwrap_or("");
    if !ind_val.is_empty() {
        if let Ok(member_id) = members::get_member_id(&ind_val.to_string()) {
            games.retain(|game| game.member_in_game(&member_id));
        }
    }

    let vs_val = game_filter.get("vs").map(|s| s.as_str()).unwrap_or("");
    if !vs_val.is_empty() {
        if let Ok(opponent_id) = members::get_member_id(&vs_val.to_string()) {
            games.retain(|game| game.member_in_game(&opponent_id));
        }
    }

    let first_min: Option<i32> = game_filter.get("first_min").and_then(|v| v.parse().ok());
    let first_max: Option<i32> = game_filter.get("first_max").and_then(|v| v.parse().ok());
    let last_min: Option<i32>  = game_filter.get("last_min").and_then(|v| v.parse().ok());
    let last_max: Option<i32>  = game_filter.get("last_max").and_then(|v| v.parse().ok());
    if first_min.is_some() || first_max.is_some() || last_min.is_some() || last_max.is_some() {
        games.retain(|game| {
            let mut scores = game.get_only_scores();
            scores.sort();
            scores.reverse();
            let first = scores[0];
            let last  = scores[3];
            first_min.map_or(true, |v| first >= v)
                && first_max.map_or(true, |v| first <= v)
                && last_min.map_or(true,  |v| last  >= v)
                && last_max.map_or(true,  |v| last  <= v)
        });
    }

    games.reverse();

    let offset: usize = game_filter.get("offset")
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    let limit: usize = game_filter.get("limit")
        .and_then(|v| v.parse().ok())
        .unwrap_or(usize::MAX);

    let games = games.into_iter().skip(offset).take(limit).collect();

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
        start <= game_time && game_time <= end
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
