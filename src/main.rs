use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use actix_web::middleware::Logger;
use env_logger::Env;

use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
struct Vote {
    name: String,
    country: String,
    voteFor: u32
}

async fn new_vote(info: web::Json<Vote>) -> impl Responder {
    format!("Hello {}!", info.name)
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
            .route("/vote/new", web::get().to(new_vote))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}