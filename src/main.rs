use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use log::info;

pub mod config;
pub mod helpers;
pub mod routes;

pub static ADDRESS: &str = "0.0.0.0:8080";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    info!("Starting app...");

    HttpServer::new(|| {
        let _cors_old = Cors::default()
            .send_wildcard()
            .allow_any_header()
            .allow_any_method()
            .expose_any_header()
            .allow_any_origin()
            .max_age(3600);

        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
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
    .bind(ADDRESS)?
    .run()
    .await
}
