use actix_web::middleware::Logger;
use actix_web::{guard, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_slog::StructuredLogger;
use slog::o;
use slog::Drain;
use slog::FnValue;
use slog_term::{CompactFormat, TermDecorator};

mod config;
mod helpers;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let decorator = TermDecorator::new().build();
    let drain = CompactFormat::new(decorator).build().fuse();

    // although not strictly required, using slog_async is always a good idea to unblock the main-thread
    // otherwise it would block until logging has been completed
    let drain = slog_async::Async::new(drain).build().fuse();

    // generic root-logger you can use for the whole application
    let root_logger = slog::Logger::root(
        drain,
        o!("version" => env!("CARGO_PKG_VERSION"), "module" => FnValue(move |info| {
            info.module().to_string()
        })
        ),
    );

    HttpServer::new(move || {
        App::new()
            .wrap(
                // initialize the structured access-logger with a child-logger/scoped logger
                StructuredLogger::new(root_logger.new(o!("log_type" => "access"))),
            )
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
