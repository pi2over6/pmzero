mod pmzero;

use std::collections::HashMap;

use actix_web::{get, post, routes, web, HttpResponse, HttpServer, Responder};

use lazy_static::lazy_static;
use tera::Tera;

lazy_static! {
    pub static ref TERA: Tera = {
        let mut tera = match Tera::new("web/*.html") {
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
    let path = std::env::var("WEB_PATH").unwrap_or(String::from("web/"));
    let listen = std::env::var("LISTEN").unwrap_or(String::from("localhost:8888"));

    println!("path: {}, listen: {}", path, listen);

    let app = move || {
        actix_web::App::new()
            .service(games)
            .service(games_filter)
            .service(ranking)
            .service(ranking_filter)
            .service(new_game_form)
            .service(new_game)
            .service(new_member)
            .service(stats)
            .service(actix_files::Files::new("/", path.clone()))
    };

    HttpServer::new(app).bind(listen)?.run().await
}

#[routes]
#[get("/")]
#[get("/ranking")]
async fn ranking(web::Query(filter): web::Query<HashMap<String, String>>) -> impl Responder {
    let ranking = pmzero::get_ranking(filter).unwrap_or(vec![]);

    let mut ctx = tera::Context::new();
    ctx.insert("ranking", &ranking);

    match TERA.render("ranking.html", &ctx) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/games")]
async fn games(web::Query(filter): web::Query<HashMap<String, String>>) -> impl Responder {
    let games = pmzero::get_games(filter).unwrap_or(vec![]);

    let mut ctx = tera::Context::new();
    ctx.insert("games", &games);

    match TERA.render("games.html", &ctx) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/games_filter")]
async fn games_filter() -> impl Responder {
    let members = pmzero::get_members().unwrap_or(vec![]);

    let mut ctx = tera::Context::new();
    ctx.insert("members", &members);

    match TERA.render("games_filter.html", &ctx) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/ranking_filter")]
async fn ranking_filter() -> impl Responder {
    let members = pmzero::get_members().unwrap_or(vec![]);

    let mut ctx = tera::Context::new();
    ctx.insert("members", &members);

    match TERA.render("ranking_filter.html", &ctx) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/new_game")]
async fn new_game_form() -> impl Responder {
    let members = pmzero::get_members().unwrap_or(vec![]);

    let mut ctx = tera::Context::new();
    ctx.insert("members", &members);

    match TERA.render("newgame.html", &ctx) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/new_game")]
async fn new_game(info: web::Form<HashMap<String, String>>) -> impl Responder {
    let result = pmzero::new_game(info.into_inner());
    match result {
        Ok(_) => web::Redirect::to("/games.html").see_other(),
        Err(e) => panic!("{}", e),
    }
}

#[post("/new_member")]
async fn new_member(info: web::Form<HashMap<String, String>>) -> impl Responder {
    let result = pmzero::new_member(info.into_inner());
    match result {
        Ok(_) => web::Redirect::to("/games.html").see_other(),
        Err(e) => panic!("{}", e),
    }
}

#[get("/stats")]
async fn stats() -> impl Responder {
    let stats = pmzero::stats().unwrap_or(HashMap::new());

    let mut ctx = tera::Context::new();
    ctx.insert("stats", &stats);

    match TERA.render("stats.html", &ctx) {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
