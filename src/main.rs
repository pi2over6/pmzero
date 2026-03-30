mod pmzero;

use std::collections::HashMap;

use actix_cors::Cors;
use actix_web::{get, post, web, HttpRequest, HttpResponse, HttpServer, Responder};

use lazy_static::lazy_static;
use tera::Tera;

lazy_static! {
    pub static ref TERA: Tera = {
        let path = std::env::var("WEB_PATH").unwrap_or(String::from("web/"));

        let mut tera = match Tera::new(&format!("{}*.html", path)) {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html", ".sql"]);
        tera
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listen = std::env::var("LISTEN").unwrap_or(String::from("localhost:8888"));

    println!("version: {}", env!("CARGO_PKG_VERSION"));

    let app = move || {
        let cors = Cors::default()
            .allowed_origin_fn(|_, _| true)
            .allowed_methods(vec!["GET", "POST"])
            .allowed_header(actix_web::http::header::CONTENT_TYPE)
            .max_age(3600);

        actix_web::App::new()
            .wrap(cors)
            // JSON API routes
            .service(api_ranking)
            .service(api_games)
            .service(api_stats)
            .service(api_members)
            .service(api_new_game)
            .service(api_new_member)
            .service(api_get_game)
            .service(api_update_game)
            .service(api_delete_game)
            .service(api_member_stat)
            // Original HTML routes (now under /old)
            .service(ranking)
            .service(games)
            .service(games_filter)
            .service(ranking_filter)
            .service(new_game_form)
            .service(new_game)
            .service(new_member_form)
            .service(new_member)
            .service(stats)
            // SvelteKit at root
            .default_service(web::to(svelte_proxy))
    };

    actix_web::rt::spawn(run_backup_task());

    HttpServer::new(app).bind(listen)?.run().await
}

fn secs_until_next_5am() -> u64 {
    use chrono::{Datelike, Local, TimeZone};
    let now = Local::now();
    let today_5am = Local
        .with_ymd_and_hms(now.year(), now.month(), now.day(), 5, 0, 0)
        .unwrap();
    let target = if now < today_5am {
        today_5am
    } else {
        today_5am + chrono::Duration::days(1)
    };
    (target - now).num_seconds().max(0) as u64
}

fn try_backup(path: &str) -> std::io::Result<()> {
    use std::time::{Duration, SystemTime};
    let meta = std::fs::metadata(path)?;
    let modified = meta.modified()?;
    let age = SystemTime::now()
        .duration_since(modified)
        .unwrap_or(Duration::MAX);
    if age <= Duration::from_secs(24 * 3600) {
        let date = chrono::Local::now().format("%Y-%m-%d");
        let backup_path = if let Some(dot) = path.rfind('.') {
            format!("{}.{}{}", &path[..dot], date, &path[dot..])
        } else {
            format!("{}.{}", path, date)
        };
        std::fs::copy(path, &backup_path)?;
        println!("Backed up {} -> {}", path, backup_path);
    }
    Ok(())
}

async fn run_backup_task() {
    loop {
        let secs = secs_until_next_5am();
        actix_web::rt::time::sleep(std::time::Duration::from_secs(secs)).await;
        for path in &[
            std::env::var("GAMES_PATH").unwrap_or_else(|_| "db/games.json".into()),
            std::env::var("MEMBERS_PATH").unwrap_or_else(|_| "db/members.json".into()),
        ] {
            if let Err(e) = try_backup(path) {
                eprintln!("Backup error for {}: {}", path, e);
            }
        }
        // avoid re-triggering within the same minute
        actix_web::rt::time::sleep(std::time::Duration::from_secs(90)).await;
    }
}

async fn svelte_proxy(req: HttpRequest, body: web::Bytes) -> HttpResponse {
    let svelte_url = std::env::var("SVELTE_URL")
        .unwrap_or_else(|_| "http://localhost:5173".to_string());
    let path = req.uri().path();
    let path = if path.is_empty() { "/" } else { path };
    let uri = match req.uri().query() {
        Some(q) => format!("{}?{}", path, q),
        None => path.to_string(),
    };
    let target = format!("{}{}", svelte_url, uri);

    let client = reqwest::Client::new();
    let method = reqwest::Method::from_bytes(req.method().as_str().as_bytes())
        .unwrap_or(reqwest::Method::GET);

    let mut proxy_req = client.request(method, &target);
    for (name, value) in req.headers() {
        if name != "host" {
            proxy_req = proxy_req.header(name.as_str(), value.as_bytes());
        }
    }

    let resp = match proxy_req.body(body.to_vec()).send().await {
        Ok(r) => r,
        Err(_) => return HttpResponse::BadGateway().body("SvelteKit server unavailable"),
    };

    let status = actix_web::http::StatusCode::from_u16(resp.status().as_u16())
        .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);
    let mut response = HttpResponse::build(status);
    for (name, value) in resp.headers() {
        if name != "transfer-encoding" && name != "connection" {
            if let Ok(v) = actix_web::http::header::HeaderValue::from_bytes(value.as_bytes()) {
                response.insert_header((name.as_str(), v));
            }
        }
    }
    match resp.bytes().await {
        Ok(bytes) => response.body(bytes),
        Err(_) => HttpResponse::InternalServerError().body("Failed to read response"),
    }
}

#[get("/old/ranking")]
async fn ranking(web::Query(filter): web::Query<HashMap<String, String>>) -> impl Responder {
    let ranking = pmzero::get_ranking(filter).unwrap_or(vec![]);

    let mut ctx = tera::Context::new();
    ctx.insert("ranking", &ranking);

    match TERA.render("ranking.html", &ctx) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/old/games")]
async fn games(web::Query(filter): web::Query<HashMap<String, String>>) -> impl Responder {
    let games = pmzero::get_games(filter).unwrap_or(vec![]);

    let mut ctx = tera::Context::new();
    ctx.insert("games", &games);

    match TERA.render("games.html", &ctx) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/old/games_filter")]
async fn games_filter() -> impl Responder {
    let members = pmzero::get_members().unwrap_or(vec![]);

    let mut ctx = tera::Context::new();
    ctx.insert("members", &members);

    match TERA.render("games_filter.html", &ctx) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/old/ranking_filter")]
async fn ranking_filter() -> impl Responder {
    let members = pmzero::get_members().unwrap_or(vec![]);

    let mut ctx = tera::Context::new();
    ctx.insert("members", &members);

    match TERA.render("ranking_filter.html", &ctx) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/old/new_game")]
async fn new_game_form() -> impl Responder {
    let members = pmzero::get_members().unwrap_or(vec![]);

    let mut ctx = tera::Context::new();
    ctx.insert("members", &members);

    match TERA.render("newgame.html", &ctx) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/old/new_game")]
async fn new_game(info: web::Form<HashMap<String, String>>) -> impl Responder {
    let result = pmzero::new_game(info.into_inner());
    match result {
        Ok(_) => web::Redirect::to("/old/games").see_other(),
        Err(e) => panic!("{}", e),
    }
}

#[get("/old/new_member")]
async fn new_member_form() -> impl Responder {
    match TERA.render("newmember.html", &tera::Context::new()) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/old/new_member")]
async fn new_member(info: web::Form<HashMap<String, String>>) -> impl Responder {
    let result = pmzero::new_member(info.into_inner());
    match result {
        Ok(_) => web::Redirect::to("/old/new_game").see_other(),
        Err(e) => panic!("{}", e),
    }
}

#[get("/old/stats")]
async fn stats() -> impl Responder {
    let stats = pmzero::stats().unwrap_or(HashMap::new());

    let mut ctx = tera::Context::new();
    ctx.insert("stats", &stats);

    match TERA.render("stats.html", &ctx) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

// ── JSON API ──────────────────────────────────────────────────────────────────

#[get("/api/ranking")]
async fn api_ranking(web::Query(filter): web::Query<HashMap<String, String>>) -> impl Responder {
    let data = pmzero::get_ranking(filter).unwrap_or(vec![]);
    HttpResponse::Ok().json(data)
}

#[get("/api/games")]
async fn api_games(web::Query(filter): web::Query<HashMap<String, String>>) -> impl Responder {
    let data = pmzero::get_games(filter).unwrap_or(vec![]);
    HttpResponse::Ok().json(data)
}

#[get("/api/stats")]
async fn api_stats() -> impl Responder {
    let data = pmzero::stats().unwrap_or(HashMap::new());
    HttpResponse::Ok().json(data)
}

#[get("/api/members")]
async fn api_members() -> impl Responder {
    let members = pmzero::get_members().unwrap_or(vec![]);
    HttpResponse::Ok().json(members)
}

#[post("/api/game")]
async fn api_new_game(info: web::Json<HashMap<String, String>>) -> impl Responder {
    match pmzero::new_game(info.into_inner()) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/api/member")]
async fn api_new_member(info: web::Json<HashMap<String, String>>) -> impl Responder {
    match pmzero::new_member(info.into_inner()) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/api/game/{id}")]
async fn api_get_game(path: web::Path<usize>) -> impl Responder {
    match pmzero::get_game_raw(path.into_inner()) {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::NotFound().body(e.to_string()),
    }
}

#[post("/api/game/{id}")]
async fn api_update_game(
    path: web::Path<usize>,
    info: web::Json<HashMap<String, String>>,
) -> impl Responder {
    match pmzero::update_game(path.into_inner(), info.into_inner()) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[actix_web::delete("/api/game/{id}")]
async fn api_delete_game(path: web::Path<usize>) -> impl Responder {
    match pmzero::delete_game(path.into_inner()) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::NotFound().body(e.to_string()),
    }
}

#[get("/api/member/{name}")]
async fn api_member_stat(path: web::Path<String>) -> impl Responder {
    match pmzero::get_member_stat(&path.into_inner()) {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::NotFound().body(e.to_string()),
    }
}
