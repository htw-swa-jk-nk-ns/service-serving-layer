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
    let resp = reqwest::get(&calculate_path).await;
    if let Err(err) = resp {
        return HttpResponse::BadRequest().body(
            format!("Berechnung answered with {}", err.status().unwrap())
        )
    }

    let resp_result = resp.unwrap();

    let mut res ;
    
    if !resp_result.status().is_success() {
        return HttpResponse::BadRequest().body(
            format!("Berechnung answered with {}", resp_result.status())
        )
    }

    res = resp_result.json::<Vec<Result>>().await;
        if let Err(err) = res {
            return HttpResponse::BadRequest().body(
                format!("Cannot parse JSON from Berechnung with Error {}", err.status().unwrap())
            )
        }
    return HttpResponse::Ok().json(res.unwrap())
}