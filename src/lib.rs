use actix_web::middleware::Logger;
use actix_web::{guard,http, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;

pub mod config;
pub mod helpers;
pub mod routes;

pub static ADDRESS: &str = "0.0.0.0:8080";
