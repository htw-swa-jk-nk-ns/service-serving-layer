use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, guard, web};
use actix_web::middleware::Logger;
use env_logger::Env;
use serde::{Serialize, Deserialize};

mod config;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(routes::get_results::get_results)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}