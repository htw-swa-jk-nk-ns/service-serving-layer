use actix_web::middleware::Logger;
use actix_web::{guard,http, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;

mod config;
mod helpers;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    HttpServer::new(|| {

        let cors = Cors::default()
              .allowed_origin("*")
              .allowed_methods(vec!["GET", "POST"])
              .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
              .allowed_header(http::header::CONTENT_TYPE)
              .max_age(3600);

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
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
