use actix_web::middleware::Logger;
use actix_web::{guard,http, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;

use service_serving_layer::*;

pub static ADRESS: &str = "0.0.0.0:8080";

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    HttpServer::new(|| {

        let cors = Cors::default()
              .send_wildcard()
              .allow_any_header()
              .allowed_methods(vec!["GET", "POST"])
              .expose_any_header()
              .allow_any_origin()
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
    .bind(ADRESS)?
    .run()
    .await
}
