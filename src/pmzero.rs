mod db;
mod games;
mod members;

use std::{collections::HashMap, error::Error};

type UserID = usize;

pub fn get_ranking(
    game_filter: HashMap<String, String>,
) -> Result<Vec<HashMap<String, String>>, Box<dyn Error>> {
    #[derive(Default)]
    struct Stat {
        point: f32,
        rank_count: [usize; 4],
    }

    let mut game_filter_ranking = game_filter.clone();
    game_filter_ranking.insert(String::from("ranking"), String::from("true"));

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

    let mut final_stats = vec![];
    for (id, stat) in stats {
        let rank_count = stat.rank_count;
        let game_count: usize = rank_count.into_iter().sum();
        let rank_avg =
            (1 * rank_count[0] + 2 * rank_count[1] + 3 * rank_count[2] + 4 * rank_count[3]) as f32
                / game_count as f32;

        final_stats.push(HashMap::from([
            (String::from("name"), members[&id].clone()),
            (String::from("point"), format!("{:.1}", stat.point)),
            (String::from("point_raw"), format!("{}", stat.point)),
            (
                String::from("point_avg"),
                format!("{:.1}", stat.point / game_count as f32),
            ),
            (String::from("games"), format!("{}", game_count)),
            (String::from("first"), rank_count[0].to_string()),
            (String::from("second"), rank_count[1].to_string()),
            (String::from("third"), rank_count[2].to_string()),
            (String::from("fourth"), rank_count[3].to_string()),
            (String::from("rank_avg"), format!("{:.2}", rank_avg)),
            (
                String::from("first_ratio"),
                format!("{:.1}", rank_count[0] as f32 / game_count as f32 * 100.0),
            ),
            (
                String::from("second_ratio"),
                format!("{:.1}", rank_count[1] as f32 / game_count as f32 * 100.0),
            ),
            (
                String::from("third_ratio"),
                format!("{:.1}", rank_count[2] as f32 / game_count as f32 * 100.0),
            ),
            (
                String::from("fourth_ratio"),
                format!("{:.1}", rank_count[3] as f32 / game_count as f32 * 100.0),
            ),
        ]));
    }

    final_stats.sort_by(|a, b| {
        let a_point = a["point_raw"].parse::<f32>().unwrap();
        let b_point = b["point_raw"].parse::<f32>().unwrap();
        b_point.partial_cmp(&a_point).unwrap()
    });

    Ok(final_stats)
}

pub fn get_members() -> Result<Vec<String>, Box<dyn Error>> {
    let members = members::members()?;

    let mut members_vec = members.values().cloned().collect::<Vec<String>>();
    members_vec.sort_unstable();

    Ok(members_vec)
}

pub fn get_games(
    game_filter: HashMap<String, String>,
) -> Result<Vec<HashMap<String, String>>, Box<dyn Error>> {
    let games = games::games_filtered(game_filter)?;
    let members = members::members()?;

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

        let row = HashMap::from([
            (String::from("recorded_at"), game.recorded_at.clone()),
            (String::from("first"), results[0].clone()),
            (String::from("second"), results[1].clone()),
            (String::from("third"), results[2].clone()),
            (String::from("fourth"), results[3].clone()),
            (
                String::from("leftover_score"),
                game.leftover_score.to_string(),
            ),
            (String::from("used_dora_count"), used_dora_count.clone()),
            (String::from("remarks"), game.remarks.clone()),
        ]);
        game_table.push(row);
    }

    Ok(game_table)
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
        remarks: info.get("remarks").unwrap_or(&String::new()).clone(),
    };

    games::append_game(&game)?;
    Ok(())
}

pub fn new_member(info: HashMap<String, String>) -> Result<(), Box<dyn Error>> {
    members::append_member(&info["new_member"])?;
    Ok(())
}

pub fn stats() -> Result<HashMap<String, String>, Box<dyn Error>> {
    let game_filter = HashMap::from([(String::from("ranking"), String::from("true"))]);
    let games = games::games_filtered(game_filter)?;

    let mut wins_by_seat = [0; 4];
    let mut lasts_by_seat = [0; 4];
    let mut lowest_score = 0;
    let mut highest_score = 0;
    let mut average_score_by_seat = [0.0; 4];

    let all_scores: Vec<[i32; 4]> = games.iter().map(|game| game.get_only_scores()).collect();
    for score in all_scores {
        let mut score_with_seat = score.into_iter().enumerate().collect::<Vec<(usize, i32)>>();

        for (seat, score) in &score_with_seat {
            average_score_by_seat[*seat] += *score as f32;
        }

        score_with_seat.sort_by(|a, b| a.1.cmp(&b.1));

        wins_by_seat[score_with_seat[3].0] += 1;
        lasts_by_seat[score_with_seat[0].0] += 1;

        if score_with_seat[3].1 > highest_score {
            highest_score = score_with_seat[3].1;
        }

        if score_with_seat[0].1 < lowest_score {
            lowest_score = score_with_seat[0].1;
        }
    }

    for ii in 0..4 {
        average_score_by_seat[ii] /= games.len() as f32;
    }

    let bankrupt = games
        .iter()
        .filter(|g| g.scores.iter().any(|s| s.1 < 0))
        .count();

    let stats = HashMap::from([
        (String::from("game_count"), format!("{}", games.len())),
        (String::from("bankrupt"), format!("{}", bankrupt)),
        (String::from("lowest_score"), format!("{}", lowest_score)),
        (String::from("highest_score"), format!("{}", highest_score)),
        (String::from("wins_east"), format!("{}", wins_by_seat[0])),
        (String::from("wins_south"), format!("{}", wins_by_seat[1])),
        (String::from("wins_west"), format!("{}", wins_by_seat[2])),
        (String::from("wins_north"), format!("{}", wins_by_seat[3])),
        (String::from("lasts_east"), format!("{}", lasts_by_seat[0])),
        (String::from("lasts_south"), format!("{}", lasts_by_seat[1])),
        (String::from("lasts_west"), format!("{}", lasts_by_seat[2])),
        (String::from("lasts_north"), format!("{}", lasts_by_seat[3])),
        (
            String::from("average_score_east"),
            format!("{:.0}", average_score_by_seat[0]),
        ),
        (
            String::from("average_score_south"),
            format!("{:.0}", average_score_by_seat[1]),
        ),
        (
            String::from("average_score_west"),
            format!("{:.0}", average_score_by_seat[2]),
        ),
        (
            String::from("average_score_north"),
            format!("{:.0}", average_score_by_seat[3]),
        ),
    ]);

    Ok(stats)
}
