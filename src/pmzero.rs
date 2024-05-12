mod db;
mod games;
mod members;

use std::{collections::HashMap, error::Error};

type UserID = usize;

pub fn get_ranking(game_filter: HashMap<String, String>) -> Result<String, Box<dyn Error>> {
    #[derive(Default)]
    struct Stat {
        point: f32,
        rank_count: [usize; 4],
    }

    let mut game_filter_ranking = game_filter.clone();
    game_filter_ranking.insert(String::from("ranking"), String::from("true"));

    for (key, value) in &game_filter_ranking {
        println!("filter key: {}, value: {}", key, value);
    }

    let games = games::games_filtered(game_filter_ranking)?;
    let members = members::members()?;
    let mut stats: HashMap<UserID, Stat> = Default::default();

    for game in &games {
        if game.non_rank_game {
            continue;
        }

        let points = game.calculate_points();
        for (ii, (point, seat)) in points.into_iter().enumerate() {
            let id = game.scores[seat as usize].0;
            let entry = stats.entry(id).or_insert(Default::default());
            entry.point += point;
            entry.rank_count[ii] += 1;
        }
    }

    #[derive(serde::Serialize)]
    struct FinalStat {
        name: String,
        point: String,
        point_raw: f32, // internal data for sorting
        point_avg: String,
        games: String,
        first: String,
        second: String,
        third: String,
        fourth: String,
        rank_avg: String,
        first_ratio: String,
        second_ratio: String,
        third_ratio: String,
        fourth_ratio: String,
    }

    let mut final_stats = vec![];
    for (id, stat) in stats {
        let rank_count = stat.rank_count;
        let game_count: usize = rank_count.into_iter().sum();
        let rank_avg =
            (1 * rank_count[0] + 2 * rank_count[1] + 3 * rank_count[2] + 4 * rank_count[3]) as f32
                / game_count as f32;

        final_stats.push(FinalStat {
            name: members[&id].clone(),
            point: format!("{:.1}", stat.point),
            point_raw: stat.point,
            point_avg: format!("{:.1}", stat.point / game_count as f32),
            games: format!("{}", game_count),
            first: rank_count[0].to_string(),
            second: rank_count[1].to_string(),
            third: rank_count[2].to_string(),
            fourth: rank_count[3].to_string(),
            rank_avg: format!("{:.2}", rank_avg),
            first_ratio: format!("{:.1}", rank_count[0] as f32 / game_count as f32 * 100.0),
            second_ratio: format!("{:.1}", rank_count[1] as f32 / game_count as f32 * 100.0),
            third_ratio: format!("{:.1}", rank_count[2] as f32 / game_count as f32 * 100.0),
            fourth_ratio: format!("{:.1}", rank_count[3] as f32 / game_count as f32 * 100.0),
        })
    }

    final_stats.sort_by(|a, b| b.point_raw.partial_cmp(&a.point_raw).unwrap());

    let result = serde_json::to_string(&final_stats)?;
    Ok(result)
}

pub fn get_members() -> Result<String, Box<dyn Error>> {
    let members = members::members()?;

    let mut members_vec = members.values().cloned().collect::<Vec<String>>();
    members_vec.sort_unstable();

    let result = serde_json::to_string(&members_vec)?;
    Ok(result)
}

pub fn get_games(game_filter: HashMap<String, String>) -> Result<String, Box<dyn Error>> {
    let games = games::games_filtered(game_filter)?;
    let members = members::members()?;

    #[derive(serde::Serialize, serde::Deserialize, Clone)]
    struct GameRow {
        recorded_at: String,
        first: String,
        second: String,
        third: String,
        fourth: String,
        leftover_score: String,
        used_dora_count: String,
        remarks: String,
    }
    let mut game_table = Vec::new();

    for game in &games {
        let seat_name = ['동', '남', '서', '북'];

        let points = game.calculate_points();

        let mut results: [String; 4] = Default::default();
        for ii in 0..4 {
            let seat_int = points[ii].1 as usize;
            let seat_string = seat_name[seat_int].clone();
            let name = members[&game.scores[seat_int].0].clone();
            let score = game.scores[seat_int].1;
            let point = points[ii].0;

            results[ii] = format!("[{}] {}: {} ({:.1})", seat_string, name, score, point);
        }

        let used_dora_count = match game.used_dora_count {
            Some(int) => int.to_string(),
            None => "".to_string(),
        };

        let row = GameRow {
            recorded_at: game.recorded_at.clone(),
            first: results[0].clone(),
            second: results[1].clone(),
            third: results[2].clone(),
            fourth: results[3].clone(),
            leftover_score: game.leftover_score.to_string(),
            used_dora_count,
            remarks: game.remarks.clone(),
        };
        game_table.push(row);
    }

    let games_filtered = serde_json::to_string(&game_table)?;
    Ok(games_filtered)
}

pub fn new_game(info: HashMap<String, String>) -> Result<(), Box<dyn Error>> {
    let player_ids = [
        members::get_member_id(&info["eastName"])?,
        members::get_member_id(&info["southName"])?,
        members::get_member_id(&info["westName"])?,
        members::get_member_id(&info["northName"])?,
    ];

    let scores = [
        str::parse(&info["eastScore"]).unwrap_or(0),
        str::parse(&info["southScore"]).unwrap_or(0),
        str::parse(&info["westScore"]).unwrap_or(0),
        str::parse(&info["northScore"]).unwrap_or(0),
    ];

    if scores.into_iter().sum::<i32>() != 100000 {
        Err("Sum of scores should be 100000")?
    }

    let game = games::Game {
        id: 0, // will be added inside append_game()
        non_rank_game: info.get("non_rank_game").unwrap_or(&String::from("off")) == "on",
        used_dora_count: Some(str::parse(&info["used_dora_count"]).unwrap_or(4)),
        recorded_at: format!("{}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S")),
        scores: player_ids
            .into_iter()
            .zip(scores.into_iter())
            .collect::<Vec<(UserID, i32)>>()
            .try_into()
            .unwrap(),
        leftover_score: str::parse(&info["leftover"]).unwrap_or(0),
        remarks: info["remarks"].clone(),
    };

    games::append_game(&game)?;
    Ok(())
}

pub fn new_member(info: HashMap<String, String>) -> Result<(), Box<dyn Error>> {
    members::append_member(&info["new_member"])?;
    Ok(())
}

pub fn stats() -> Result<String, Box<dyn Error>> {
    let game_filter = HashMap::from([(String::from("ranking"), String::from("true"))]);
    let games = games::games_filtered(game_filter)?;

    #[derive(serde::Serialize)]
    struct Stats {
        game_count: usize,
        bankrupt: usize,
        highest_score: i32,
        wins_by_seat: [usize; 4],
        lasts_by_seat: [usize; 4],
        average_score_by_seat: [f32; 4],
    }

    let mut wins_by_seat = [0; 4];
    let mut lasts_by_seat = [0; 4];
    let mut highest_score = 0;
    let mut average_score_by_seat = [0.0; 4];

    let all_scores: Vec<[i32; 4]> = games.iter().map(|game| game.get_only_scores()).collect();
    for score in all_scores {
        let mut score_with_seat = score.into_iter().enumerate().collect::<Vec<(usize, i32)>>();

        for (seat, score) in &score_with_seat {
            average_score_by_seat[*seat] += *score as f32;
        }

        score_with_seat.sort();

        wins_by_seat[score_with_seat[0].0] += 1;
        lasts_by_seat[score_with_seat[3].0] += 1;

        if score_with_seat[0].1 > highest_score {
            highest_score = score_with_seat[0].1;
        }
    }

    for ii in 0..4 {
        average_score_by_seat[ii] /= games.len() as f32;
    }

    let stats = Stats {
        game_count: games.len(),
        bankrupt: games.iter().filter(|g| g.scores.iter().any(|s| s.1 < 0)).count(),
        highest_score,
        wins_by_seat,
        lasts_by_seat,
        average_score_by_seat,
    };

    let result = serde_json::to_string(&stats)?;
    Ok(result)
}
