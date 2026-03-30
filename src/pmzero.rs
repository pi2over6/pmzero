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

        let mut results: Vec<HashMap<String, String>> = Vec::new();
        for ii in 0..4 {
            let seat_int = points[ii].1 as usize;
            let seat_string = seat_name[seat_int].to_string();
            let name = members[&game.scores[seat_int].0].clone();
            let score = game.scores[seat_int].1;
            let point = points[ii].0;

            results.push(HashMap::from([
                (String::from("name"), name),
                (String::from("score"), score.to_string()),
                (String::from("seat"), seat_string),
                (String::from("point"), format!("{:.1}", point)),
            ]));
        }

        let rank_keys = ["first", "second", "third", "fourth"];
        let used_dora_count = match game.used_dora_count {
            Some(int) => int.to_string(),
            None => "".to_string(),
        };

        let mut row = HashMap::from([
            (String::from("id"), game.id.to_string()),
            (String::from("recorded_at"), game.recorded_at.clone()),
            (String::from("leftover_score"), game.leftover_score.to_string()),
            (String::from("used_dora_count"), used_dora_count),
            (String::from("remarks"), game.remarks.clone()),
        ]);
        for (ii, key) in rank_keys.iter().enumerate() {
            row.insert(format!("{}_name", key), results[ii]["name"].clone());
            row.insert(format!("{}_score", key), results[ii]["score"].clone());
            row.insert(format!("{}_seat", key), results[ii]["seat"].clone());
            row.insert(format!("{}_point", key), results[ii]["point"].clone());
        }
        game_table.push(row);
    }

    Ok(game_table)
}

pub fn get_game_raw(id: usize) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let games = db::load_games()?;
    let game = games.iter().find(|g| g.id == id)
        .ok_or_else(|| format!("Game {} not found", id))?;
    let members = members::members()?;
    let seat_names = ["east", "south", "west", "north"];
    let mut map = HashMap::new();
    map.insert("id".to_string(), game.id.to_string());
    map.insert("recorded_at".to_string(), game.recorded_at.clone());
    map.insert("leftover".to_string(), game.leftover_score.to_string());
    map.insert("used_dora_count".to_string(),
        game.used_dora_count.map(|v| v.to_string()).unwrap_or_default());
    map.insert("remarks".to_string(), game.remarks.clone());
    map.insert("non_rank_game".to_string(),
        if game.non_rank_game { "on".to_string() } else { "off".to_string() });
    for (i, seat) in seat_names.iter().enumerate() {
        let (member_id, score) = game.scores[i];
        let name = members.get(&member_id).cloned().unwrap_or_default();
        map.insert(format!("{}Name", seat), name);
        map.insert(format!("{}Score", seat), score.to_string());
    }
    Ok(map)
}

pub fn update_game(id: usize, info: HashMap<String, String>) -> Result<(), Box<dyn Error>> {
    let mut games = db::load_games()?;
    let pos = games.iter().position(|g| g.id == id)
        .ok_or_else(|| format!("Game {} not found", id))?;

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

    games[pos].scores = player_ids.into_iter().zip(scores.into_iter())
        .collect::<Vec<(UserID, i32)>>().try_into().unwrap();
    games[pos].leftover_score = str::parse(&info["leftover"]).unwrap_or(0);
    games[pos].used_dora_count = Some(str::parse(&info["used_dora_count"]).unwrap_or(4));
    games[pos].remarks = info.get("remarks").unwrap_or(&String::new()).clone();
    games[pos].non_rank_game = info.get("non_rank_game").unwrap_or(&String::from("off")) == "on";

    db::save_games(&games)?;
    Ok(())
}

pub fn get_member_stat(name: &str) -> Result<serde_json::Value, Box<dyn Error>> {
    use serde_json::json;

    let member_id = members::get_member_id(&name.to_string())?;
    let all_members = members::members()?;
    let all_games = db::load_games()?;

    // games this member participated in, oldest-first
    let member_games: Vec<&db::Game> = all_games.iter()
        .filter(|g| g.scores.iter().any(|(id, _)| *id == member_id))
        .collect();

    // ranking games only
    let rank_games: Vec<&db::Game> = member_games.iter()
        .filter(|g| !g.non_rank_game)
        .copied()
        .collect();

    let game_count = rank_games.len();
    let mut rank_count = [0usize; 4];
    let mut seat_rank_count = [[0usize; 4]; 4]; // seat_rank_count[seat][rank]
    let mut total_point: f32 = 0.0;
    let mut scores_vec: Vec<i32> = vec![];
    let mut scores_by_seat: [Vec<i32>; 4] = Default::default();
    let mut wins_by_seat = [0usize; 4];
    let mut lasts_by_seat = [0usize; 4];
    let mut bankrupt = 0usize;
    let mut co_player_count: HashMap<usize, usize> = HashMap::new();

    let mut recent: Vec<serde_json::Value> = vec![];

    for game in &rank_games {
        let points = game.calculate_points();
        // find this member's rank and seat
        let seat = game.scores.iter().position(|(id, _)| *id == member_id).unwrap();
        let rank = points.iter().position(|(_, s)| *s as usize == seat).unwrap();
        let score = game.scores[seat].1;
        let point = points[rank].0;

        rank_count[rank] += 1;
        seat_rank_count[seat][rank] += 1;
        total_point += point;
        scores_vec.push(score);
        scores_by_seat[seat].push(score);
        if rank == 0 { wins_by_seat[seat] += 1; }
        if rank == 3 { lasts_by_seat[seat] += 1; }
        if score < 0 { bankrupt += 1; }

        for (i, (id, _)) in game.scores.iter().enumerate() {
            if i != seat {
                *co_player_count.entry(*id).or_insert(0) += 1;
            }
        }

        recent.push(json!({
            "date": game.recorded_at,
            "rank": rank + 1,
            "score": score,
            "point": format!("{:.1}", point),
        }));
    }

    // recent: last 10 reversed (newest first)
    let recent_slice: Vec<serde_json::Value> = recent.iter().rev().take(10).cloned().collect();

    let rank_avg = if game_count > 0 {
        (1 * rank_count[0] + 2 * rank_count[1] + 3 * rank_count[2] + 4 * rank_count[3]) as f32
            / game_count as f32
    } else { 0.0 };

    let avg_score = if game_count > 0 {
        scores_vec.iter().sum::<i32>() as f32 / game_count as f32
    } else { 0.0 };
    let best_score = scores_vec.iter().copied().max().unwrap_or(0);
    let worst_score = scores_vec.iter().copied().min().unwrap_or(0);

    // seat stats: games played per seat and 1st place count
    let seat_names = ["동", "남", "서", "북"];
    let seat_stats: Vec<serde_json::Value> = (0..4).map(|s| {
        let played: usize = seat_rank_count[s].iter().sum();
        let first = seat_rank_count[s][0];
        let last  = seat_rank_count[s][3];
        let avg_s = if scores_by_seat[s].is_empty() { 0.0 } else {
            scores_by_seat[s].iter().sum::<i32>() as f32 / scores_by_seat[s].len() as f32
        };
        json!({
            "seat": seat_names[s],
            "played": played,
            "first": first,
            "last": last,
            "avg_score": format!("{:.0}", avg_s),
            "first_ratio": if played > 0 { format!("{:.1}", first as f32 / played as f32 * 100.0) } else { "-".to_string() },
        })
    }).collect();

    // co-players sorted by count desc, top 5
    let mut co_vec: Vec<(usize, usize)> = co_player_count.into_iter().collect();
    co_vec.sort_by(|a, b| b.1.cmp(&a.1));
    let co_players: Vec<serde_json::Value> = co_vec.iter().take(5).map(|(id, cnt)| {
        json!({
            "name": all_members.get(id).cloned().unwrap_or_default(),
            "count": cnt,
        })
    }).collect();

    // point trend: cumulative point over all rank games (oldest first)
    let mut cum = 0.0f32;
    let point_trend: Vec<serde_json::Value> = rank_games.iter().map(|game| {
        let points = game.calculate_points();
        let seat = game.scores.iter().position(|(id, _)| *id == member_id).unwrap();
        let rank = points.iter().position(|(_, s)| *s as usize == seat).unwrap();
        cum += points[rank].0;
        json!({
            "date": game.recorded_at,
            "point": format!("{:.1}", cum),
        })
    }).collect();

    Ok(json!({
        "name": name,
        "games": game_count,
        "bankrupt": bankrupt,
        "point": format!("{:.1}", total_point),
        "point_avg": if game_count > 0 { format!("{:.1}", total_point / game_count as f32) } else { "0.0".to_string() },
        "first": rank_count[0], "second": rank_count[1], "third": rank_count[2], "fourth": rank_count[3],
        "first_ratio":  format!("{:.1}", if game_count > 0 { rank_count[0] as f32 / game_count as f32 * 100.0 } else { 0.0 }),
        "second_ratio": format!("{:.1}", if game_count > 0 { rank_count[1] as f32 / game_count as f32 * 100.0 } else { 0.0 }),
        "third_ratio":  format!("{:.1}", if game_count > 0 { rank_count[2] as f32 / game_count as f32 * 100.0 } else { 0.0 }),
        "fourth_ratio": format!("{:.1}", if game_count > 0 { rank_count[3] as f32 / game_count as f32 * 100.0 } else { 0.0 }),
        "rank_avg": format!("{:.2}", rank_avg),
        "avg_score": format!("{:.0}", avg_score),
        "best_score": best_score,
        "worst_score": worst_score,
        "wins_by_seat":  wins_by_seat,
        "lasts_by_seat": lasts_by_seat,
        "seat_stats": seat_stats,
        "co_players": co_players,
        "recent": recent_slice,
        "point_trend": point_trend,
    }))
}

pub fn delete_game(id: usize) -> Result<(), Box<dyn Error>> {
    let mut games = db::load_games()?;
    let pos = games.iter().position(|g| g.id == id)
        .ok_or_else(|| format!("Game {} not found", id))?;
    games.remove(pos);
    db::save_games(&games)?;
    Ok(())
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
