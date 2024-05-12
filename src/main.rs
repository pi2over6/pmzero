mod pmzero;

use std::collections::HashMap;

use actix_web::{
    get, http::header::ContentType, post, web, HttpResponse, HttpServer, Responder
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let path = std::env::var("WEB_PATH").unwrap_or(String::from("web/"));
    let listen = std::env::var("LISTEN").unwrap_or(String::from("localhost:8888"));

    println!("path: {}, listen: {}", path, listen);

    let app = move || {
        actix_web::App::new()
            .service(index)
            .service(ranking)
            .service(games)
            .service(members)
            .service(new_game)
            .service(new_member)
            .service(actix_files::Files::new("/", path.clone()))
    };

    HttpServer::new(app).bind(listen)?.run().await
}

#[get("/")]
async fn index() -> impl Responder {
    web::Redirect::to("/ranking.html").permanent()
}

#[get("/ranking")]
async fn ranking(web::Query(filter): web::Query<HashMap<String, String>>) -> impl Responder {
    let result = pmzero::get_ranking(filter);
    match result {
        Ok(ranking) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(ranking),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/games")]
async fn games(web::Query(filter): web::Query<HashMap<String, String>>) -> impl Responder {
    let result = pmzero::get_games(filter);
    match result {
        Ok(games) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(games),
        Err(e) => {
            if let Some(_) = e.downcast_ref::<chrono::format::ParseError>() {
                HttpResponse::BadRequest().body(e.to_string())
            } else {
                HttpResponse::InternalServerError().body(e.to_string())
            }
        }
    }
}

#[get("/members")]
async fn members() -> impl Responder {
    let result = pmzero::get_members();
    match result {
        Ok(members) => HttpResponse::Ok().content_type(ContentType::json()).body(members),
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
    let result = pmzero::stats();
    match result {
        Ok(body) => HttpResponse::Ok().content_type(ContentType::json()).body(body),
        Err(e) => panic!("{}", e),
    }
}
