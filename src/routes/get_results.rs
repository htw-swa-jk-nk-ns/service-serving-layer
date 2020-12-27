use reqwest::Request;
use serde::{Serialize, Deserialize};
use actix_web::{get, HttpResponse, HttpServer, Responder, client::ConnectError, guard, web};
use std::collections::HashMap;
use crate::config::get_config;
#[derive(Serialize, Deserialize)]
pub struct Result {
    candidate: String,
    totalVotes: u32
}

#[get("/getResults")]
pub async fn get_results() -> HttpResponse {
    let calculate_path = format!("{}{}",get_config().calculate_adress, "results");
    let res = crate::routes::get::get::<Vec<Result>>(get_config().calculate_adress, "results".to_string()).await;
    match res {
        Ok(result) => {return HttpResponse::Ok().json(result);}
        Err(err) => {return HttpResponse::BadRequest().body(err)}
    }
}