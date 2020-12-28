use crate::config::get_config;
use actix_web::{get, HttpResponse};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Result {
    country: String,
    winner: String,
    votes: u32,
}

#[get("/getLandslideVictory")]
pub async fn exec() -> HttpResponse {
    let res = crate::helpers::get::<Result>(
        get_config().calculate_adress,
        "getLandslideVictory".to_string(),
    )
    .await;
    match res {
        Ok(result) => {
            return HttpResponse::Ok().json(result);
        }
        Err(err) => return HttpResponse::BadRequest().body(err),
    }
}
