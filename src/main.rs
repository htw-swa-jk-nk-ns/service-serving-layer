use actix_web::middleware::Logger;
use actix_web::{guard, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use env_logger::Env;

mod config;
mod helpers;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(routes::get_results::exec)
            .service(routes::get_candidates_by_country::exec)
            .service(routes::get_closest_vote_difference::exec)
            .service(routes::get_landslide_victory::exec)
            .service(routes::get_results_by_country::exec)
            .service(routes::get_top_five_candidates_by_vote::exec)
            .service(routes::post_vote::exec)
            .service(routes::get_all_data::exec)
            .service(routes::test::exec)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
